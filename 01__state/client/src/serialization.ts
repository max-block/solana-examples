import lo from "buffer-layout"
import BN from "bn.js"

export interface Counter {
  counter: number
  value: BN
}

export interface Settings {
  admin: number[]
  inc_step: number
  dec_step: number
}

enum CounterIxOrder {
  Inc = 0,
  Dec = 1,
  UpdateSettings = 2,
}

const counterSchema = lo.struct([lo.u32("counter"), lo.ns64("value")])
const settingsSchema = lo.struct([
  lo.seq(lo.u8(), 32, "admin"),
  lo.u32("inc_step"),
  lo.u32("dec_step"),
])

export function encodeCounter(counter: number, value: BN): Buffer {
  const b = Buffer.alloc(4 + 8)
  counterSchema.encode({ counter, value }, b)
  return b
}

export function decodeCounter(data: Buffer): Counter {
  return counterSchema.decode(data)
}

export function decodeSettings(data: Buffer): Settings {
  return settingsSchema.decode(data)
}

export function encodeIncIx(): Buffer {
  return Buffer.from([CounterIxOrder.Inc])
}

export function encodeDecIx(): Buffer {
  return Buffer.from([CounterIxOrder.Dec])
}

export function encodeUpdateSettingsIx(
  admin: Uint8Array,
  inc_step: number,
  dec_step: number
): Buffer {
  const schema = lo.struct([lo.seq(lo.u8(), 32, "admin"), lo.u32("inc_step"), lo.u32("dec_step")])
  const b = Buffer.alloc(32 + 4 + 4)
  schema.encode({ admin, inc_step, dec_step }, b)
  return Buffer.from([CounterIxOrder.UpdateSettings, ...b])
}
