let visibleElements = [];
const intersectionObserver = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (visibleElements.includes(entry.target) && !entry.isIntersecting) {
        visibleElements = visibleElements.filter((el) => el != entry.target);
      }
      if (!visibleElements.includes(entry.target) && entry.isIntersecting) {
        visibleElements.push(entry.target);
      }
    });
    if (visibleElements.length == 0) {
      return;
    }
    let topElement = visibleElements[0];
    visibleElements.forEach((el) => {
      if (
        el.getBoundingClientRect().top < topElement.getBoundingClientRect().top
      ) {
        topElement = el;
      }
    });
    document
      .querySelectorAll("nav a.current")
      .forEach((el) => el.classList.remove("current"));
    const targetNavLink = document.querySelector(
      `nav a[href="#${topElement.id}"]`,
    );
    // window.history.replaceState(null, null, "#" + topElement.id);
    if (targetNavLink) {
      targetNavLink.classList.add("current");
    }
  },
  {
    rootMargin: "0px",
    threshold: 0.01,
  },
);
document.querySelectorAll("h2, h3").forEach((target) => {
  intersectionObserver.observe(target);
});
