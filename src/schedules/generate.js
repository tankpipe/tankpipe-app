import { invoke } from "@tauri-apps/api/core"

const generate = async (dateStr) => {
    if (dateStr) {
        console.log("generating to " + dateStr)
        await invoke('generate', {date: {date: dateStr}}).then(resolved, rejected)
    }
}

const getEndDate = async () => {
    let tempDate = await invoke('end_date')
    if (tempDate) {
        return tempDate.date
    } else {
        return null
    }
}

function resolved() {
    console.log("Generation complete.")
}

function rejected(result) {
    console.log(result)
}

export {generate, getEndDate}