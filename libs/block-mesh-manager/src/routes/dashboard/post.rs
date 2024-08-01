use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::daily_stat::get_daily_stats_by_user_id::get_daily_stats_by_user_id;
use crate::database::invite_code::get_number_of_users_invited::get_number_of_users_invited;
use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::database::perks::get_user_perks::get_user_perks;
use crate::database::task::count_user_tasks_by_status::count_user_tasks_by_status;
use crate::database::uptime_report::get_user_uptimes::get_user_uptimes;
use crate::domain::aggregate::AggregateName;
use crate::domain::task::TaskStatus;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::utils::points::calc_points;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{DailyStatForDashboard, DashboardResponse};
use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::Level;

#[tracing::instrument(name = "dashboard post", skip(auth), level = "trace",  err(level = Level::TRACE))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DashboardResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let overall_task_count =
        count_user_tasks_by_status(&mut transaction, &user.id, TaskStatus::Completed).await?;
    let number_of_users_invited = get_number_of_users_invited(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    let uptime_aggregate = get_or_create_aggregate_by_user_and_name_no_transaction(
        &pool,
        AggregateName::Uptime,
        user.id,
    )
    .await
    .map_err(Error::from)?;
    let overall_uptime = uptime_aggregate.value.as_f64().unwrap_or_default();
    let user_invite_code = get_user_latest_invite_code(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;

    let uptimes = get_user_uptimes(&mut transaction, user.id, 2).await?;
    let connected = if uptimes.len() == 2 {
        let diff = uptimes[0].created_at - uptimes[1].created_at;
        if diff.num_seconds() < 60 {
            true
        } else {
            false
        }
    } else {
        false
    };
    let perks: Vec<f64> = get_user_perks(&mut transaction, user.id)
        .await?
        .into_iter()
        .map(|i| i.multiplier)
        .collect();
    let daily_stats = get_daily_stats_by_user_id(&mut transaction, &user.id)
        .await?
        .into_iter()
        .map(|i| {
            let points = calc_points(i.uptime, i.tasks_count, &perks);
            DailyStatForDashboard {
                tasks_count: i.tasks_count,
                uptime: i.uptime,
                points,
                day: i.day,
            }
        })
        .rev()
        .collect();
    let points = calc_points(overall_uptime, overall_task_count, &perks);
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(DashboardResponse {
        points,
        number_of_users_invited,
        invite_code: user_invite_code.invite_code,
        connected,
        daily_stats,
    }))
}
