{// DOM utils:
const el = (sel, par = document) => par.querySelector(sel);
const els = (sel, par = document) => par.querySelectorAll(sel);
const elNew = (tag, prop = {}) => Object.assign(document.createElement(tag), prop);

// Resizable
let isResizing = false;
const resizableGrid = (elParent, idx) => {

  const isVert = elParent.classList.contains("panes-v");
  const elsPanes = elParent.querySelectorAll(":scope > .pane");

  let fr = [...elsPanes].map(() => 1 / elsPanes.length);
  let elPaneCurr = null;
  let paneIndex = -1;
  let frStart = 0;

  const frToCSS = () => {
    elParent.style[isVert ? "grid-template-rows" : "grid-template-columns"] = fr.join("fr ") + "fr";
  };

  const pointerDown = (evt) => {
    if (isResizing || !evt.target.closest(".gutter")) return;
    isResizing = true;
    elPaneCurr = evt.currentTarget;
    elPaneCurr.setPointerCapture(evt.pointerId);
    fr = [...elsPanes].map((elPane) => isVert ? elPane.clientHeight / elParent.clientHeight : elPane.clientWidth / elParent.clientWidth);
    paneIndex = [...elsPanes].indexOf(elPaneCurr);
    frStart = fr[paneIndex];
    frNext = fr[paneIndex + 1];
    addEventListener("pointermove", pointerMove);
    addEventListener("pointerup", pointerUp);
  };

  const pointerMove = (evt) => {
    evt.preventDefault();
    const paneBCR = elPaneCurr.getBoundingClientRect();
    const parentSize = isVert ? elParent.clientHeight : elParent.clientWidth;
    const pointer = {
      x: Math.max(0, Math.min(evt.clientX - paneBCR.left, elParent.clientWidth)),
      y: Math.max(0, Math.min(evt.clientY - paneBCR.top, elParent.clientHeight))
    };
    const frRel = pointer[isVert ? "y" : "x"] / parentSize;
    const frDiff = frStart - frRel;
    fr[paneIndex] = Math.max(0.05, frRel);
    fr[paneIndex + 1] = Math.max(0.05, frNext + frDiff);
    frToCSS();
  };

  const pointerUp = (evt) => {
    removeEventListener("pointermove", pointerMove);
    removeEventListener("pointerup", pointerUp);
    elPaneCurr.releasePointerCapture(evt.pointerId);
    isResizing = false;
  };

  [...elsPanes].slice(0, -1).forEach((elPane, i) => {
    elPane.append(elNew("div", {
      className: "gutter"
    }));
    elPane.addEventListener("pointerdown", pointerDown);
  });
  frToCSS();
};

els(".panes").forEach(resizableGrid);
}
