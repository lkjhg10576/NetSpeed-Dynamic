<template>
    <div class="stats-chart-wrapper">
        <!-- 图例 -->
        <div class="stats-legend">
            <span class="legend-item">
                <span class="legend-dot" :style="{ background: upColor }"></span>
                上传 (MB)
            </span>
            <span class="legend-item">
                <span class="legend-dot" :style="{ background: downColor }"></span>
                下载 (MB)
            </span>
        </div>
        <canvas ref="canvasRef" class="stats-canvas"></canvas>
        <!-- Tooltip -->
        <div v-if="tooltip.visible" class="stats-tooltip" :style="{ left: tooltip.x + 'px', top: tooltip.y + 'px' }">
            <div class="tooltip-title">{{ tooltip.title }}</div>
            <div class="tooltip-row"><span class="tooltip-dot" :style="{ background: upColor }"></span>上传: {{ tooltip.up }} MB</div>
            <div class="tooltip-row"><span class="tooltip-dot" :style="{ background: downColor }"></span>下载: {{ tooltip.down }} MB</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, reactive } from 'vue';

const props = defineProps<{
    days: string[];
    upData: number[];
    downData: number[];
    chartType: 'bar' | 'line';
}>();

const canvasRef = ref<HTMLCanvasElement | null>(null);
let ctx: CanvasRenderingContext2D | null = null;

const tooltip = reactive({
    visible: false,
    x: 0,
    y: 0,
    title: '',
    up: '0',
    down: '0',
    index: -1,
});

const getColors = () => {
    const isDark = document.documentElement.classList.contains('dark-theme');
    return {
        textColor: isDark ? '#94a3b8' : '#64748b',
        splitLine: isDark ? '#383c41' : '#f1f5f9',
        upColor: isDark ? '#60a5fa' : '#3b82f6',
        downColor: isDark ? '#34d399' : '#10b981',
    };
};

const upColor = ref('#3b82f6');
const downColor = ref('#10b981');

const barRects: { x: number; y: number; w: number; h: number; index: number; series: 'up' | 'down' }[] = [];

const draw = () => {
    if (!canvasRef.value) return;

    const canvas = canvasRef.value;
    const rect = canvas.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;

    if (canvas.width !== rect.width * dpr || canvas.height !== rect.height * dpr) {
        canvas.width = rect.width * dpr;
        canvas.height = rect.height * dpr;
    }

    ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.scale(dpr, dpr);

    const w = rect.width;
    const h = rect.height;
    const padding = { top: 4, bottom: 28, left: 10, right: 10 };
    const chartW = w - padding.left - padding.right;
    const chartH = h - padding.top - padding.bottom;

    ctx.clearRect(0, 0, w, h);

    const colors = getColors();
    upColor.value = colors.upColor;
    downColor.value = colors.downColor;

    const { days, upData, downData, chartType } = props;
    if (!days.length) return;

    const maxVal = Math.max(...upData, ...downData, 1);

    barRects.length = 0;

    // 绘制 X 轴标签
    ctx.fillStyle = colors.textColor;
    ctx.font = '11px -apple-system, sans-serif';
    ctx.textAlign = 'center';
    const categoryW = chartW / days.length;
    for (let i = 0; i < days.length; i++) {
        const x = padding.left + categoryW * i + categoryW / 2;
        ctx.fillText(days[i], x, h - padding.bottom + 18);
    }

    // 绘制水平网格线
    const gridLines = 4;
    ctx.strokeStyle = colors.splitLine;
    ctx.lineWidth = 1;
    ctx.setLineDash([4, 4]);
    for (let i = 0; i <= gridLines; i++) {
        const y = padding.top + (chartH / gridLines) * i;
        ctx.beginPath();
        ctx.moveTo(padding.left, y);
        ctx.lineTo(w - padding.right, y);
        ctx.stroke();

        // Y 轴标签
        const val = ((maxVal * (gridLines - i)) / gridLines).toFixed(1);
        ctx.fillStyle = colors.textColor;
        ctx.textAlign = 'left';
        ctx.fillText(val, padding.left, y - 4);
    }
    ctx.setLineDash([]);

    if (chartType === 'bar') {
        // 柱状图
        const groupWidth = categoryW * 0.6;
        const barWidth = groupWidth / 2 - 2;
        const barGap = 4;

        for (let i = 0; i < days.length; i++) {
            const groupX = padding.left + categoryW * i + (categoryW - groupWidth) / 2;

            // 上传柱
            const upH = (upData[i] / maxVal) * chartH;
            const upX = groupX;
            const upY = padding.top + chartH - upH;

            ctx.fillStyle = colors.upColor;
            ctx.beginPath();
            ctx.roundRect(upX, upY, barWidth, upH, [3, 3, 0, 0]);
            ctx.fill();
            barRects.push({ x: upX, y: upY, w: barWidth, h: upH, index: i, series: 'up' });

            // 下载柱
            const downH = (downData[i] / maxVal) * chartH;
            const downX = groupX + barWidth + barGap;
            const downY = padding.top + chartH - downH;

            ctx.fillStyle = colors.downColor;
            ctx.beginPath();
            ctx.roundRect(downX, downY, barWidth, downH, [3, 3, 0, 0]);
            ctx.fill();
            barRects.push({ x: downX, y: downY, w: barWidth, h: downH, index: i, series: 'down' });
        }
    } else {
        // 折线图
        const drawLine = (data: number[], color: string) => {
            const points: { x: number; y: number }[] = [];
            for (let i = 0; i < data.length; i++) {
                const x = padding.left + categoryW * i + categoryW / 2;
                const y = padding.top + chartH - (data[i] / maxVal) * chartH;
                points.push({ x, y });
            }

            // 渐变填充
            const gradient = ctx!.createLinearGradient(0, padding.top, 0, padding.top + chartH);
            gradient.addColorStop(0, color + '30');
            gradient.addColorStop(1, color + '00');

            ctx!.beginPath();
            ctx!.moveTo(points[0].x, padding.top + chartH);
            ctx!.lineTo(points[0].x, points[0].y);
            for (let i = 0; i < points.length - 1; i++) {
                const p0 = points[Math.max(i - 1, 0)];
                const p1 = points[i];
                const p2 = points[i + 1];
                const p3 = points[Math.min(i + 2, points.length - 1)];
                const tension = 0.3;
                const cp1x = p1.x + (p2.x - p0.x) * tension;
                const cp1y = p1.y + (p2.y - p0.y) * tension;
                const cp2x = p2.x - (p3.x - p1.x) * tension;
                const cp2y = p2.y - (p3.y - p1.y) * tension;
                ctx!.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, p2.x, p2.y);
            }
            ctx!.lineTo(points[points.length - 1].x, padding.top + chartH);
            ctx!.closePath();
            ctx!.fillStyle = gradient;
            ctx!.fill();

            // 线条
            ctx!.beginPath();
            ctx!.moveTo(points[0].x, points[0].y);
            for (let i = 0; i < points.length - 1; i++) {
                const p0 = points[Math.max(i - 1, 0)];
                const p1 = points[i];
                const p2 = points[i + 1];
                const p3 = points[Math.min(i + 2, points.length - 1)];
                const tension = 0.3;
                const cp1x = p1.x + (p2.x - p0.x) * tension;
                const cp1y = p1.y + (p2.y - p0.y) * tension;
                const cp2x = p2.x - (p3.x - p1.x) * tension;
                const cp2y = p2.y - (p3.y - p1.y) * tension;
                ctx!.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, p2.x, p2.y);
            }
            ctx!.strokeStyle = color;
            ctx!.lineWidth = 2;
            ctx!.stroke();

            // 数据点
            for (const p of points) {
                ctx!.beginPath();
                ctx!.arc(p.x, p.y, 3, 0, Math.PI * 2);
                ctx!.fillStyle = color;
                ctx!.fill();
                ctx!.strokeStyle = '#fff';
                ctx!.lineWidth = 1.5;
                ctx!.stroke();
            }
        };

        drawLine(downData, colors.downColor);
        drawLine(upData, colors.upColor);
    }
};

// 鼠标悬停 tooltip
const handleMouseMove = (e: MouseEvent) => {
    if (!canvasRef.value) return;
    const rect = canvasRef.value.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (props.chartType === 'bar') {
        let found = false;
        for (const bar of barRects) {
            if (x >= bar.x && x <= bar.x + bar.w && y >= bar.y && y <= bar.y + bar.h) {
                tooltip.visible = true;
                tooltip.x = e.clientX - rect.left + 12;
                tooltip.y = e.clientY - rect.top - 10;
                tooltip.title = props.days[bar.index];
                tooltip.up = props.upData[bar.index].toFixed(2);
                tooltip.down = props.downData[bar.index].toFixed(2);
                tooltip.index = bar.index;
                found = true;
                break;
            }
        }
        if (!found) tooltip.visible = false;
    } else {
        // 折线图：检测最近的数据点
        const padding = { top: 4, bottom: 28, left: 10, right: 10 };
        const chartW = rect.width - padding.left - padding.right;
        const categoryW = chartW / props.days.length;
        let nearest = -1;
        let minDist = Infinity;
        for (let i = 0; i < props.days.length; i++) {
            const cx = padding.left + categoryW * i + categoryW / 2;
            const dist = Math.abs(x - cx);
            if (dist < minDist && dist < categoryW / 2) {
                minDist = dist;
                nearest = i;
            }
        }
        if (nearest >= 0) {
            tooltip.visible = true;
            tooltip.x = e.clientX - rect.left + 12;
            tooltip.y = e.clientY - rect.top - 10;
            tooltip.title = props.days[nearest];
            tooltip.up = props.upData[nearest].toFixed(2);
            tooltip.down = props.downData[nearest].toFixed(2);
            tooltip.index = nearest;
        } else {
            tooltip.visible = false;
        }
    }
};

const handleMouseLeave = () => {
    tooltip.visible = false;
};

watch(() => [props.upData, props.downData, props.chartType, props.days], () => {
    requestAnimationFrame(draw);
}, { deep: true });

let themeObserver: MutationObserver | null = null;

onMounted(() => {
    draw();
    canvasRef.value?.addEventListener('mousemove', handleMouseMove);
    canvasRef.value?.addEventListener('mouseleave', handleMouseLeave);

    themeObserver = new MutationObserver(() => {
        requestAnimationFrame(draw);
    });
    themeObserver.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ['class'],
    });
});

onUnmounted(() => {
    themeObserver?.disconnect();
    canvasRef.value?.removeEventListener('mousemove', handleMouseMove);
    canvasRef.value?.removeEventListener('mouseleave', handleMouseLeave);
});

const resize = () => {
    requestAnimationFrame(draw);
};

defineExpose({ resize });
</script>

<style scoped>
.stats-chart-wrapper {
    position: relative;
    width: 100%;
    height: 100%;
}

.stats-legend {
    display: flex;
    gap: 16px;
    justify-content: center;
    margin-bottom: 4px;
    font-size: 12px;
    color: var(--speed-label, #64748b);
}

.legend-item {
    display: flex;
    align-items: center;
    gap: 6px;
}

.legend-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    display: inline-block;
}

.stats-canvas {
    width: 100%;
    height: calc(100% - 24px);
    display: block;
}

.stats-tooltip {
    position: absolute;
    background: var(--card-bg, #fff);
    border: 1px solid var(--card-border, #e2e8f0);
    border-radius: 8px;
    padding: 8px 12px;
    font-size: 12px;
    pointer-events: none;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    z-index: 10;
    white-space: nowrap;
}

.tooltip-title {
    font-weight: 600;
    margin-bottom: 4px;
    color: var(--card-h3-color, #334155);
}

.tooltip-row {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--speed-label, #64748b);
}

.tooltip-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    display: inline-block;
}
</style>
