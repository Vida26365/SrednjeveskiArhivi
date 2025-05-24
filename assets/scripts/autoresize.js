{
  const autoresize = (element) => {
    // Force the browser to recalculate the height
    element.style.overflow = 'hidden'
    element.style.height = 'auto'

    // Set the height to fit the content
    element.style.height = (element.scrollHeight + 5) + 'px'
    element.scrollTop = element.scrollHeight
    element.style.overflow = 'initial'
  }

  document.querySelectorAll('textarea.autoresize').forEach((textarea) => {
    textarea.addEventListener('input', () => autoresize(textarea), true)

    let userResizing = false
    textarea.addEventListener('pointerdown', () => { userResizing = true }, true)
    textarea.addEventListener('pointerup', () => { userResizing = false }, true)

    const observer = new ResizeObserver(() => !userResizing ? autoresize(textarea) : null)
    observer.observe(textarea)
  })
}
