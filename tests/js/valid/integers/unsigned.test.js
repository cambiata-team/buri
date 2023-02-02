import { fiveMinusTwo, one, onePlusTwo, two, twoTimesThree } from "@tests/js/valid/integers/unsigned.mjs"

import { expect, it } from "bun:test"

it("a literal with the value of 1 should be equal to 1", () => {
    expect(one.valueOf()).toBe(1)
})

it("a literal with the value of 2 should be equal to 2", () => {
    expect(two.valueOf()).toBe(2)
})

it("literal one plus literal two should equal three", () => {
    expect(onePlusTwo.valueOf()).toBe(3)
})

it("literal five minus literal two should equal three", () => {
    expect(fiveMinusTwo.valueOf()).toBe(3)
})

it("literal two times literal three should equal six", () => {
    expect(twoTimesThree.valueOf()).toBe(6)
})