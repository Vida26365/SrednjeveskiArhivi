{
  const createElement = (tag, props = {}) => Object.assign(document.createElement(tag), props)

  const getDefaultFractions = (panes) => {
    let fr = []

    let reservedSize = 0
    let setCount = 0
    let unsetCount = 0

    panes.forEach(el => {
      const spec = parseFloat(el.dataset.defaultSize)
      if (!isNaN(spec)) {
        fr.push(spec)
        reservedSize += spec
        setCount++
      } else {
        fr.push(null)
        unsetCount++
      }
    })

    const availableSize = 1 - reservedSize
    const defaultSize = unsetCount ? availableSize / unsetCount : 0
    return fr.map(val => (val === null ? defaultSize : val))
  }

  const minFraction = 0.0
  const minSize = 26

  let isResizing = false

  const resizableGrid = (grid) => {
    const isDisabled = grid.classList.contains('panes-disabled')
    const isVertical = grid.classList.contains('panes-vertical')

    const panes = [...grid.querySelectorAll(':scope > .pane')]

    let fr = getDefaultFractions(panes)
    let frStart = 0
    let frNext = 0

    let currPaneEl = null
    let currPaneIx = -1

    const frToCSS = () => {
      const vals = fr.flatMap((element, index) =>
        index < fr.length - 1
          ? [`${element}fr`, 'calc(var(--spacing) * 2)']
          : [`${element}fr`],
      )

      grid.style[isVertical ? 'grid-template-rows' : 'grid-template-columns'] = vals.join(' ')
    }

    const pointerDown = (event) => {
      if (isResizing) return
      isResizing = true

      currPaneEl = event.currentTarget.previousElementSibling
      currPaneIx = panes.indexOf(currPaneEl)

      currPaneEl.setPointerCapture(event.pointerId)
      event.preventDefault()

      frStart = fr[currPaneIx]
      frNext = fr[currPaneIx + 1]

      addEventListener('pointermove', pointerMove)
      addEventListener('pointerup', pointerUp)
    }

    const pointerMove = (event) => {
      event.preventDefault()

      const paneBCR = currPaneEl.getBoundingClientRect()
      const parentSize = isVertical ? grid.clientHeight : grid.clientWidth
      const pointerPos = isVertical ? event.clientY - paneBCR.top : event.clientX - paneBCR.left
      const clampedPos = Math.max(0, Math.min(pointerPos, parentSize))

      const desiredCurrent = clampedPos / parentSize

      const minCurrentFrac = Math.max(minSize / parentSize, minFraction)
      const minNextFrac = Math.max(minSize / parentSize, minFraction)

      let newCurrent = Math.max(desiredCurrent, minCurrentFrac)
      let newNext = frNext + (frStart - newCurrent)

      if (newNext < minNextFrac) {
        newNext = minNextFrac
        newCurrent = frStart + frNext - newNext
        newCurrent = Math.max(newCurrent, minCurrentFrac)
      }

      fr[currPaneIx] = newCurrent
      fr[currPaneIx + 1] = newNext

      const total = fr.reduce((a, b) => a + b, 0)
      fr = fr.map(val => val / total)

      frToCSS()
    }

    const pointerUp = (event) => {
      currPaneEl.releasePointerCapture(event.pointerId)
      event.preventDefault()

      removeEventListener('pointermove', pointerMove)
      removeEventListener('pointerup', pointerUp)

      isResizing = false
    }

    panes.slice(0, -1).forEach((pane, _) => {
      let gutter = createElement('div', { className: 'gutter' })
      if (!isDisabled) gutter.addEventListener('pointerdown', pointerDown)
      pane.after(gutter)
    })

    frToCSS()
  }

  document.querySelectorAll('.panes').forEach(resizableGrid)
}
