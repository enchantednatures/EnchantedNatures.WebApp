window.onload = function () {
  (function (document, window, index) {
    "use strict";
    // get nav element
    let element = document.getElementById("sidebar-nav");
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
    console.log(element);
    console.log("setup");

    window.addEventListener("scroll", function () {
      console.log("scroooolllling");
      elWidth = element.offsetWidth;
      dWidth = document.body.offsetWidth;
      wWidth = window.innerWidth;
      wScrollCurrent = window.pageXOffset;
      wScrollDiff = wScrollBefore - wScrollCurrent;
      elLeft = parseInt(window.getComputedStyle(element).left) + wScrollDiff;
      if (wScrollCurrent <= 0) {
        // scrolled to the very left; element sticks to the left
        element.style.left = "0px";
      } else if (wScrollDiff > 0) {
        // scrolled up; element slides in
        element.style.left = (elLeft > 0 ? 0 : elLeft) + "px";
      } else if (wScrollDiff < 0) {
        // scrolled down
        element.style.left =
          (Math.abs(elLeft) > elWidth ? -elWidth : elLeft) + "px";
      }
      console.log(element.style.left);
      wScrollBefore = wScrollCurrent;
    });
  })(document, window, 0);
};
