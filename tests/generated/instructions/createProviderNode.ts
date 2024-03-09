/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category CreateProviderNode
 * @category generated
 */
export const createProviderNodeStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */
}>(
  [['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)]],
  'CreateProviderNodeInstructionArgs'
)
/**
 * Accounts required by the _createProviderNode_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [_writable_] providerNode
 * @category Instructions
 * @category CreateProviderNode
 * @category generated
 */
export type CreateProviderNodeInstructionAccounts = {
  signer: web3.PublicKey
  providerNode: web3.PublicKey
  systemProgram?: web3.PublicKey
  rent?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const createProviderNodeInstructionDiscriminator = [
  136, 202, 211, 227, 62, 97, 88, 25,
]

/**
 * Creates a _CreateProviderNode_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category CreateProviderNode
 * @category generated
 */
export function createCreateProviderNodeInstruction(
  accounts: CreateProviderNodeInstructionAccounts,
  programId = new web3.PublicKey('GzscdwWG2FwpA6iqB6yYKEESvvw773c1iAzmJatXLcve')
) {
  const [data] = createProviderNodeStruct.serialize({
    instructionDiscriminator: createProviderNodeInstructionDiscriminator,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.providerNode,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}
