(() => {
    const SCROLL_KEY = "soc-scroll";
    const initial_scroll = localStorage.getItem(SCROLL_KEY);
    if (initial_scroll != null) {
        window.scroll(
            window.scrollX,
            initial_scroll
        );
    }

    window.addEventListener("scroll", () => {
        localStorage.setItem(SCROLL_KEY, window.scrollY)
    })
})();