export const disabledItemsByIndex = (entries, getId, wrapId = (id) => ({ id })) => {
  const selectedIds = []
  const seen = new Set()

  for (const entry of entries || []) {
    const id = getId?.(entry)
    if (!id || seen.has(id)) continue
    seen.add(id)
    selectedIds.push(id)
  }

  return (entries || []).map((entry) => {
    const mineId = getId?.(entry)
    const otherIds = selectedIds.filter((id) => id !== mineId)
    return wrapId ? otherIds.map(wrapId) : otherIds
  })
}

export const disabledItemsKeyByIndex = (disabledItems) => {
  return (disabledItems || []).map((list) =>
    (list || [])
      .map((item) => (typeof item === 'string' ? item : item?.id))
      .filter(Boolean)
      .sort()
      .join('|')
  )
}

