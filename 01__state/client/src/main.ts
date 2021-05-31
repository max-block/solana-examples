import {App} from "./app";

async function main() {
    const app = new App()
    await app.init()
    await app.createCounterAccount()
    await app.createSettingsAccount()
    await app.incCounter()

    let counter = await app.readCounterAccount()
    console.log("counter", counter)


}

main().then(() => process.exit(0)).catch(err => console.error(err))