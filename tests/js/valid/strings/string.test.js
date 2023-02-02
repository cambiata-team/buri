import { emptyString } from "@tests/js/valid/strings/string.mjs"

import { expect, it } from "bun:test"

it("empty string literal should have the value of an empty string", () => {
    expect(emptyString.valueOf()).toBe("")
})