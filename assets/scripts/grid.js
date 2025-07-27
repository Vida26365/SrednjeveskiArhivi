{
  const createElement = (tag, props = {}) => Object.assign(document.createElement(tag), props)

  const getDefaultFractions = (elsPanes) => {
    let fr = []

    let reservedSize = 0
    let setCount = 0
    let unsetCount = 0

    elsPanes.forEach(el => {
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

  const getMinWidth = (elem) => {
    const origMinWidth = elem.style.minWidth
    const origWidth = elem.style.width

    elem.style.minWidth = 'min-content'
    elem.style.width = 0

    const calcMinWidth = elem.getBoundingClientRect().width

    elem.style.minWidth = origMinWidth
    elem.style.width = origWidth

    return calcMinWidth
  }

  const minFraction = 0.01

  let isResizing = false

  const resizableGrid = (elParent) => {
    const isVertical = elParent.classList.contains('panes-v')
    const elsPanes = [...elParent.querySelectorAll(':scope > .pane')]

    const minSizes = elsPanes.map(elPane => isVertical ? 0 : getMinWidth(elPane))

    let fr = getDefaultFractions(elsPanes)
    let frStart = 0
    let frNext = 0

    let currPaneEl = null
    let currPaneIx = -1

    const frToCSS = () => {
      elParent.style[isVertical ? 'grid-template-rows' : 'grid-template-columns'] = fr.join('fr ') + 'fr'
    }

    const pointerDown = (event) => {
      if (isResizing || !event.target.closest('.gutter')) return
      isResizing = true

      currPaneEl = event.currentTarget
      currPaneIx = elsPanes.indexOf(currPaneEl)

      currPaneEl.setPointerCapture(event.pointerId)
      event.preventDefault()

      fr = elsPanes.map((elPane) => isVertical ? elPane.clientHeight / elParent.clientHeight : elPane.clientWidth / elParent.clientWidth)

      frStart = fr[currPaneIx]
      frNext = fr[currPaneIx + 1]

      addEventListener('pointermove', pointerMove)
      addEventListener('pointerup', pointerUp)
    }

    const pointerMove = (event) => {
      event.preventDefault()

      const paneBCR = currPaneEl.getBoundingClientRect()
      const parentSize = isVertical ? elParent.clientHeight : elParent.clientWidth
      const pointerPos = isVertical ? event.clientY - paneBCR.top : event.clientX - paneBCR.left
      const clampedPos = Math.max(0, Math.min(pointerPos, parentSize))

      const desiredCurrent = clampedPos / parentSize

      const minCurrentFrac = isVertical ? minFraction : Math.max(minSizes[currPaneIx] / parentSize, minFraction)
      const minNextFrac = isVertical ? minFraction : Math.max(minSizes[currPaneIx + 1] / parentSize, minFraction)

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

    const windowResized = () => {
      const parentSize = isVertical ? elParent.clientHeight : elParent.clientWidth

      fr = elsPanes.map((elPane, i) => {
        const minSize = isVertical ? 0 : minSizes[i]
        const currentSize = (isVertical ? elPane.clientHeight : elPane.clientWidth)

        return Math.max(currentSize / parentSize, Math.max(minSize / parentSize, minFraction))
      })

      const total = fr.reduce((a, b) => a + b, 0)
      fr = fr.map(val => val / total)

      frToCSS()
    }

    elsPanes.slice(0, -1).forEach((elPane, i) => {
      elPane.append(createElement('div', { className: 'gutter' }))
      elPane.addEventListener('pointerdown', pointerDown)
    })

    const observer = new ResizeObserver(windowResized)
    observer.observe(elParent)

    frToCSS()
  }

  document.querySelectorAll('.panes').forEach(resizableGrid)
}
