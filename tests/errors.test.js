import { Errors } from "../src/errors"

it('can be in error', async () => {
    let errors = new Errors()
    expect(errors.hasErrors()).toBe(false)
    expect(errors.isInError("field1")).toBe(false)
    errors.addError("field1", "Fix me")
    expect(errors.hasErrors()).toBe(true)
    expect(errors.isInError("field1")).toBe(true)
    expect(errors.isInError("field2")).toBe(false)
    const msgs = errors.getErrorMessages()
    expect(msgs[0]).toBe("Fix me")
})