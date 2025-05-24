{
  const createElement = (tag, props = {}) => Object.assign(document.createElement(tag), props)

  let isResizing = false

  const resizableGrid = (elParent) => {
    const isVertical = elParent.classList.contains('panes-v')
    const elsPanes = elParent.querySelectorAll(':scope > .pane')

    let fr = [...elsPanes].map(() => 1 / elsPanes.length)
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
      currPaneIx = [...elsPanes].indexOf(currPaneEl)

      currPaneEl.setPointerCapture(event.pointerId)
      event.preventDefault()

      fr = [...elsPanes].map((elPane) => isVertical ? elPane.clientHeight / elParent.clientHeight : elPane.clientWidth / elParent.clientWidth)

      frStart = fr[currPaneIx]
      frNext = fr[currPaneIx + 1]

      addEventListener('pointermove', pointerMove)
      addEventListener('pointerup', pointerUp)
    }

    const pointerMove = (event) => {
      event.preventDefault()

      const paneBCR = currPaneEl.getBoundingClientRect()
      const parentSize = isVertical ? elParent.clientHeight : elParent.clientWidth

      const pointer = {
        x: Math.max(0, Math.min(event.clientX - paneBCR.left, elParent.clientWidth)),
        y: Math.max(0, Math.min(event.clientY - paneBCR.top, elParent.clientHeight)),
      }

      const frRel = Math.max(0.1, pointer[isVertical ? 'y' : 'x'] / parentSize)
      const frDiff = frStart - frRel

      fr[currPaneIx] = frRel
      fr[currPaneIx + 1] = Math.max(0.1, frNext + frDiff)

      frToCSS()
    }

    const pointerUp = (event) => {
      currPaneEl.releasePointerCapture(event.pointerId)
      event.preventDefault()

      removeEventListener('pointermove', pointerMove)
      removeEventListener('pointerup', pointerUp)

      isResizing = false
    }

    {
      [...elsPanes].slice(0, -1).forEach((elPane, i) => {
        elPane.append(createElement('div', { className: 'gutter' }))
        elPane.addEventListener('pointerdown', pointerDown)
      })
    }

    frToCSS()
  }

  document.querySelectorAll('.panes').forEach(resizableGrid)
}
