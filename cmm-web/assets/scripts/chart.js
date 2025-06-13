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

    chart = new Chart(ctx, {
      type: "radar",
      options: {
        plugins: {
          legend: {
            display: false,
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
            scales: {
              r: {
                max: 5,
                min: 0,
              },
            },
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
    document.querySelectorAll("#domain-scores tbody tr").forEach((tr) => {
      labels.push(tr.querySelector("td:first-child").innerText);
      values.push(
        Number.parseFloat(tr.querySelector("td:last-child").innerText)
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
