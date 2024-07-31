import AsyncStorage from '@react-native-async-storage/async-storage'

export async function storeData(key: string, value: string): Promise<void> {
  try {
    await AsyncStorage.setItem('my-key', value)
  } catch (e) {
    console.error(`storeData Error: ${key} => ${value} | error ${e}`)
  }
}

export async function getData(key: string): Promise<string | null | undefined> {
  try {
    return await AsyncStorage.getItem(key)
  } catch (e) {
    console.error(`getData Error: ${key} | error ${e}`)
  }
}