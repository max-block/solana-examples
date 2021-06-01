import {App} from "./app";

async function main() {
    const app = new App()
    await app.init()
    await app.updateCounterSettings(app.adminKeypair.publicKey.toBytes(), 19, 98)
    await app.createCounterAccount()

    console.log("counter initially", await app.readCounterAccount())

    await app.incCounter()
    console.log("counter after inc", await app.readCounterAccount())


    await app.decCounter()
    console.log("counter after dec", await app.readCounterAccount())

    console.log("settings", await app.readSettingsAccount())


}

main().then(() => process.exit(0)).catch(err => console.error(err))