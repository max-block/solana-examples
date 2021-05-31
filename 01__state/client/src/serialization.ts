import lo from "buffer-layout"
import BN from "bn.js"

export interface Counter {
    last_user: number[]
    value: BN
}

export interface Setting {
    admin: number[]
    inc_step: number
    dec_step: number
}

const counterSchema = lo.struct([lo.seq(lo.u8(), 32, "last_user"), lo.ns64("value")])
const settingsSchema = lo.struct([lo.seq(lo.u8(), 32, "admin"), lo.u32("inc_step"), lo.u32("dec_step")])

export function encodeSettings(admin: Uint8Array, inc_step: number, dec_step: number) {
    const b = Buffer.alloc(32 + 4 + 4)
    settingsSchema.encode({"admin": Array.from(admin), inc_step, dec_step}, b)
    return b
}

export function encodeCounter(last_user: Uint8Array, value: BN): Buffer {
    console.log("encodeCounter", last_user, value)
    const b = Buffer.alloc(32 + 8)
    counterSchema.encode({"last_user": Array.from(last_user), value}, b)
    return b
}

export function decodeCounter(data: Buffer): Counter {
    return counterSchema.decode(data)
}

export function encodeIncIx(): Buffer {
    return Buffer.from([0])
}

export function encodeUpdateSettingsIx(inc_step: number, dec_step: number): Buffer {
    const schema = lo.struct([lo.u32("inc_step"), lo.u32("dec_step")])
    const b = Buffer.alloc(4 + 4)
    schema.encode({inc_step, dec_step}, b)
    console.log("z", Array.from(b), inc_step, dec_step)
    return Buffer.from([3, ...b])
}