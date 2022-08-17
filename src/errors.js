class Errors{
    constructor() {
        this.errors = []
        this.fields = {}
    }

    addError(field, message) {
        if (!this.errors.includes(message)) {
            this.errors.push(message)
        }
        this.fields[field] = message
    }

    isInError(field) {
        return field in this.fields
    }

    getErrorMessages() {
        return this.errors;
    }

    hasErrors() {
        return this.errors.length > 0;
    }
}


export {Errors}