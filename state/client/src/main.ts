import { App } from "./app"

async function main() {
  const app = new App()
  await app.init()

  await app.updateCounterSettings(app.adminKeypair.publicKey.toBytes(), 19, 98)
  await app.createCounterAndInc()
  await app.decCounter()

  console.log("counter", await app.readCounterAccount())
  console.log("settings", await app.readSettingsAccount())
}

main()
  .then(() => process.exit(0))
  .catch((err) => console.error(err))
