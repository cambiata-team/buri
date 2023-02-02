import { helloString, emptyString } from "@tests/js/valid/strings/string.mjs"

import { expect, it } from "bun:test"

it("empty string literal should have the value of an empty string", () => {
    expect(emptyString.valueOf()).toBe("")
})

it("hello string literal should have the value of hello", () => {
    expect(helloString.valueOf()).toBe("hello")
})

it("newline string literal should have the value of newline", () => {
    expect(helloString.valueOf()).toBe("\n")
})
