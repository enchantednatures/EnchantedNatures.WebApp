(
  function () {
  "use strict";

  function scrollHorizontally(e) {
    e = window.event || e;
    var delta = Math.max(-1, Math.min(1, (e.wheelDelta || -e.detail)));
    document.documentElement.scrollLeft -= (delta * 120);
    document.body.scrollLeft -= (delta * 120); // Multiplied by 40
    e.preventDefault();
  }

  if (window.addEventListener) {
    // IE9, Chrome, Safari, Opera
    window.addEventListener("mousewheel", scrollHorizontally, false);
    // Firefox
    window.addEventListener("DOMMouseScroll", scrollHorizontally, false);
  } else {
    // IE 6/7/8
    window.attachEvent("onmousewheel", scrollHorizontally);
  }
})();

window.onload = function () {
  (
    function (document, window, index) {
      'use strict';
      // get nav element
      var elSelector = '.sidebar',
        element = document.querySelector(elSelector);
      // exit if we don't have nav element
      if (!element) return true;
      // defaults
      var elWidth = 0,
        elLeft = 0,
        dWidth = 0,
        wWidth = 0,
        wScrollCurrent = 0,
        wScrollBefore = 0,
        wScrollDiff = 0;
      window.addEventListener('scroll', function () {
        elWidth = element.offsetWidth;
        dWidth = document.body.offsetWidth;
        wWidth = window.innerWidth;
        wScrollCurrent = window.pageXOffset;
        wScrollDiff = wScrollBefore - wScrollCurrent;
        elLeft = parseInt(window.getComputedStyle(element).left) + wScrollDiff;
        if (wScrollCurrent <= 0) { // scrolled to the very left; element sticks to the left
          element.style.left = '0px';
        } else if (wScrollDiff > 0) { // scrolled up; element slides in
          element.style.left = (elLeft > 0 ? 0 : elLeft) + 'px';
        } else if (wScrollDiff < 0) // scrolled down
        {
          element.style.left = (Math.abs(elLeft) > elWidth ? -elWidth : elLeft) + 'px';
        }
        console.log(element.style.left)
        wScrollBefore = wScrollCurrent;
      });
    }(document, window, 0)
  );
}
