import {
  BarController,
  BarElement,
  CategoryScale,
  Chart,
  Legend,
  LinearScale,
  Title,
  Tooltip,
} from 'chart.js'
import ChartDataLabels from 'chartjs-plugin-datalabels'
import { defineComponent, onMounted } from 'vue'
import { useRef } from 'vue-jsx-vapor'

Chart.register(
  BarController,
  BarElement,
  CategoryScale,
  LinearScale,
  Tooltip,
  Legend,
  Title,
  ChartDataLabels,
)

export default defineComponent(({ title = 'Operations Per Second' }) => {
  const canvasRef = useRef(null)

  onMounted(async () => {
    const res = await fetch('/bench-results.json')
    const data: any[] = await res.json()

    // 准备数据轴
    const labels = data.map((i) => i.name)
    const babelValues = data.map((i) => i.babel)
    const oxcValues = data.map((i) => i.oxc)
    const multipliers: string[] = data.map((i) => i.multiplierText)

    const ctx = canvasRef.value!.getContext('2d')!

    new Chart(ctx, {
      type: 'bar',
      data: {
        labels,
        datasets: [
          {
            label: 'Oxc',
            data: oxcValues,
            backgroundColor: '#10b981',
            barPercentage: 1,
            categoryPercentage: 0.5,
          },
          {
            label: 'Babel',
            data: babelValues,
            backgroundColor: '#cbd5e1',
            barPercentage: 1,
            categoryPercentage: 0.5,
          },
        ],
      },
      options: {
        indexAxis: 'y',
        responsive: true,
        maintainAspectRatio: false,
        layout: { padding: { right: 60 } },
        plugins: {
          title: {
            display: true,
            padding: {
              bottom: 20,
            },
            font: {
              size: 18,
            },
            text: title,
          },
          legend: { position: 'bottom' },
          tooltip: {
            mode: 'index',
          },
          datalabels: {
            align: 'end',
            anchor: 'end',
            font: { weight: 'bold', size: 12 },
            formatter: (value, ctx) => {
              if (ctx.dataset.label === 'Oxc') {
                return multipliers[ctx.dataIndex]
              }
              return ''
            },
            color: '#10b981',
          },
        },
        scales: {
          x: {
            display: true,
            position: 'top',
            grid: { color: '#f3f4f6' },
          },
          y: {
            grid: { display: false },
          },
        },
      },
    })
  })

  return () => (
    <div style="height: 300px; max-width: 900px; margin: 0 auto; padding: 0 20px;">
      <canvas ref={canvasRef}></canvas>
      <p style="text-align: center; font-size: 0.9em; opacity: 0.6; margin-top: 10px;">
        * Benchmarks run on M1 Pro, Node.js v23 (
        <a
          href="https://github.com/vuejs/vue-jsx-vapor/blob/main/benchmark/bench.js"
          target="_blank"
        >
          Benchmark code
        </a>
        ) <br />
      </p>
    </div>
  )
})
