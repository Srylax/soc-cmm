(() => {
  const ctx = document.querySelector("canvas");

  let chart = null;

  function initChart(loaded = 0) {
    if (typeof Chart == "undefined" && chart == null) {
      if (loaded > 10) {
        console.error("Failed to load Chart class");
        return;
      }
      setTimeout(() => {
        initChart(loaded + 1);
      }, 500);
      return;
    }
    if (chart != null) {
      return;
    }

    chart = new Chart(ctx, {
      type: "radar",
      options: {
        plugins: {
          legend: {
            display: false,
          },
        },
        scales: {
          r: {
            max: 5,
            min: 0,
          },
        },
        aspectRatio: 2,
      },
      data: {
        labels: [],
        datasets: [
          {
            data: [],
            fill: true,
          },
        ],
      },
    });
    updateSpiderChart();
  }

  function updateSpiderChart() {
    if (chart == null) {
      initChart();
      return;
    }
    const labels = [];
    const values = [];
    document.querySelectorAll("#domain-scores [data-aspect-value]").forEach((aspect) => {
      labels.push(aspect.innerText);
      values.push(
        Number.parseFloat(aspect.dataset.aspectValue)
      );
    });
    chart.data.labels = labels;
    chart.data.datasets[0].data = values;
    chart.update();
  }

  document.addEventListener("updateChart", () => {
    updateSpiderChart();
  });

  initChart();
})();
