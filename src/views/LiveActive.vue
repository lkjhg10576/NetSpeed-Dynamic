<template>
    <div class="gallery-layout-wrapper">
        <div class="gallery-header">
            <div class="title-wrapper">
                <h1 class="art-title">Live<span class="art-accent">Active</span><span class="art-dot">.</span></h1>
                <div class="title-sub-row">
                    <span class="title-badge">实时活动枢纽</span>
                    <span class="subtitle-divider">/</span>
                    <span class="subtitle-text">选择模块 · 激活体验</span>
                </div>
            </div>
        </div>

        <transition name="fade">
            <button v-show="canScrollLeft" class="scroll-btn scroll-btn-left" @click="scrollToLeft">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                    stroke-linejoin="round">
                    <polyline points="15 18 9 12 15 6"></polyline>
                </svg>
            </button>
        </transition>

        <transition name="fade">
            <button v-show="canScrollRight" class="scroll-btn scroll-btn-right" @click="scrollToRight">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                    stroke-linejoin="round">
                    <polyline points="9 18 15 12 9 6"></polyline>
                </svg>
            </button>
        </transition>

        <div class="gallery-track" ref="trackRef" @scroll="checkScroll"
            :class="{ 'mask-left': canScrollLeft, 'mask-right': canScrollRight }">

            <div v-for="item in activities" :key="item.id" class="gallery-card"
                :class="{ 'is-active': activeId === item.id }" :style="{ '--accent-color': item.accent }"
                @click="handleCardClick(item.id)">

                <div class="card-hero">
                    <div class="hero-top-row">
                        <div class="pro-icon" v-html="item.icon"></div>
                        <div v-if="item.id === 'pomodoro'" class="pomodoro-controls" @click.stop>
                            <button class="pomo-btn reset-btn" @click="handleResetBtn">重置</button>
                            <button class="pomo-btn toggle-btn" :class="{ 'is-running': item.enabled }"
                                @click="handleToggleBtn(item)">
                                {{ item.enabled ? '暂停' : (isStarted ? '继续' : '启动') }}
                            </button>
                        </div>
                        <label v-else class="custom-switch" @click.stop>
                            <input type="checkbox" v-model="item.enabled" :disabled="item.disable">
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class="hero-text">
                        <h3>{{ item.title }}</h3>
                        <p>{{ item.desc }}</p>
                    </div>
                </div>

                <div class="card-body" v-if="activeId === item.id">
                    <transition name="fade-up" appear>
                        <div class="body-content">
                            <template v-if="item.id === 'pomodoro'">
                                <div class="pro-setting-item mt-10"
                                    style="flex-direction: column; align-items: flex-start; gap: 12px; border-bottom: none; padding-bottom: 0;">
                                    <div class="pro-meta">
                                        <span class="pro-title">专注时长</span>
                                    </div>

                                    <div
                                        style="width: 100%; display: flex; justify-content: space-between; align-items: center;">
                                        <div v-if="!isEditingPomodoro" class="time-display"
                                            style="font-size: 28px; font-weight: 800; color: var(--accent-color); padding: 0; text-align: left;">
                                            {{ formattedPomodoroTime }}
                                        </div>

                                        <div v-else class="time-input-group" style="padding: 0; gap: 6px;">
                                            <input type="text" v-model="inputH" class="time-input"
                                                style="width: 48px; font-size: 16px; padding: 4px;" placeholder="时" />
                                            <span class="time-separator" style="font-size: 18px;">:</span>
                                            <input type="text" v-model="inputM" class="time-input"
                                                style="width: 48px; font-size: 16px; padding: 4px;" placeholder="分" />
                                            <span class="time-separator" style="font-size: 18px;">:</span>
                                            <input type="text" v-model="inputS" class="time-input"
                                                style="width: 48px; font-size: 16px; padding: 4px;" placeholder="秒" />
                                        </div>

                                        <button v-if="!isEditingPomodoro" class="time-edit-btn"
                                            style="padding: 4px 12px;" :disabled="isStarted"
                                            @click="startEditTime">设置</button>
                                        <button v-else class="time-edit-btn" style="padding: 4px 12px;"
                                            @click="saveTime">保存</button>
                                    </div>
                                </div>

                                <div style="border-bottom: 1px dashed var(--control-border); margin: 6px 0;"></div>

                                <div class="pro-setting-item" style="padding-top: 6px;">
                                    <div class="pro-meta">
                                        <span class="pro-title">免打扰模式</span>
                                    </div>
                                    <label class="custom-switch mini"><input type="checkbox"><span
                                            class="slider"></span></label>
                                </div>
                            </template>

                            <template v-else>
                                <div class="pro-coming-soon">
                                    <div class="loader-line"></div>
                                    <p>模块部署中 SYSTEM_PENDING</p>
                                </div>
                            </template>
                        </div>
                    </transition>
                </div>
            </div>

            <div class="spacer"></div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick, computed } from 'vue';
import { emit, listen } from '@tauri-apps/api/event';
import {
    NSD_POMODORO_ACTIVE, NSD_POMODORO_STARTED,
    NSD_POMODORO_VISIBLE, NSD_POMODORO_TIME
} from '../constants/storageKeys';

const activities = ref([
    {
        id: 'pomodoro',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>',
        title: '专注番茄钟',
        desc: '沉浸工作时间管理',
        accent: '#ff4757',
        enabled: localStorage.getItem(NSD_POMODORO_ACTIVE) === 'true',
        disable: false
    },
    {
        id: 'sysmsg',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>',
        title: '系统动态感知',
        desc: '实时捕捉软硬件生态变化',
        accent: '#ff4757',
        enabled: false,
        disable: true
    },
    {
        id: 'printer',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 6 2 18 2 18 9"></polyline><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path><rect x="6" y="14" width="12" height="8"></rect></svg>',
        title: '打印机队列',
        desc: '批量打印进度状态',
        accent: '#ff4757',
        enabled: false,
        disable: true
    },
]);

// 启动或暂停番茄钟
const handleToggleBtn = async (item: any) => {
    if (item.id === 'pomodoro') {
        if (isEditingPomodoro.value) {
            saveTime();
        }

        const nextState = !item.enabled;
        item.enabled = nextState;

        if (nextState) {
            isStarted.value = true;
            localStorage.setItem(NSD_POMODORO_STARTED, 'true');
            localStorage.setItem(NSD_POMODORO_ACTIVE, 'true');
            localStorage.setItem(NSD_POMODORO_VISIBLE, 'true');
        } else {
            localStorage.setItem(NSD_POMODORO_ACTIVE, 'false');
        }

        await emit('live-activity-toggle', {
            id: item.id,
            enabled: nextState
        });
    } else {
        item.enabled = !item.enabled;
        await emit('live-activity-toggle', {
            id: item.id,
            enabled: item.enabled
        });
    }
};

// 重置番茄钟
const handleResetBtn = async () => {
    const pomoItem = activities.value.find(a => a.id === 'pomodoro');
    if (pomoItem) {
        pomoItem.enabled = false;
        isStarted.value = false;

        localStorage.setItem(NSD_POMODORO_ACTIVE, 'false');
        localStorage.setItem(NSD_POMODORO_STARTED, 'false');
        localStorage.setItem(NSD_POMODORO_VISIBLE, 'false');

        const savedTime = Number(localStorage.getItem(NSD_POMODORO_TIME)) || 1500;
        pomodoroTime.value = savedTime;

        await emit('live-activity-toggle', {
            id: 'pomodoro',
            enabled: false,
            time: savedTime,
            isReset: true
        });
    }
};

const pomodoroTime = ref(Number(localStorage.getItem(NSD_POMODORO_TIME)) || 1500);
const isEditingPomodoro = ref(false);
const isStarted = ref(localStorage.getItem(NSD_POMODORO_STARTED) === 'true');

const inputH = ref('00');
const inputM = ref('25');
const inputS = ref('00');

const formattedPomodoroTime = computed(() => {
    const h = Math.floor(pomodoroTime.value / 3600).toString().padStart(2, '0');
    const m = Math.floor((pomodoroTime.value % 3600) / 60).toString().padStart(2, '0');
    const s = (pomodoroTime.value % 60).toString().padStart(2, '0');
    return `${h}:${m}:${s}`;
});

const filterTimeInput = (val: string) => {
    const onlyNum = val.replace(/[^\d]/g, '');
    return onlyNum.slice(0, 3);
};

watch(inputH, (newVal) => { inputH.value = filterTimeInput(newVal); });
watch(inputM, (newVal) => { inputM.value = filterTimeInput(newVal); });
watch(inputS, (newVal) => { inputS.value = filterTimeInput(newVal); });

const startEditTime = () => {
    inputH.value = Math.floor(pomodoroTime.value / 3600).toString();
    inputM.value = Math.floor((pomodoroTime.value % 3600) / 60).toString();
    inputS.value = (pomodoroTime.value % 60).toString();
    isEditingPomodoro.value = true;
};

const saveTime = () => {
    const h = Number(inputH.value) || 0;
    const m = Number(inputM.value) || 0;
    const s = Number(inputS.value) || 0;
    const totalSeconds = h * 3600 + m * 60 + s;
    pomodoroTime.value = totalSeconds;
    localStorage.setItem(NSD_POMODORO_TIME, totalSeconds.toString());
    isEditingPomodoro.value = false;
    isStarted.value = false;
    localStorage.setItem(NSD_POMODORO_STARTED, 'false');
    emit('live-activity-toggle', { id: 'pomodoro', enabled: false, time: totalSeconds });
};

const activeId = ref('pomodoro');
const trackRef = ref<HTMLElement | null>(null);

const canScrollLeft = ref(false);
const canScrollRight = ref(true);

const checkScroll = () => {
    if (!trackRef.value) return;
    const { scrollLeft, clientWidth } = trackRef.value;

    canScrollLeft.value = scrollLeft > 1;

    const cards = trackRef.value.querySelectorAll('.gallery-card');
    if (cards.length > 0) {
        const lastCard = cards[cards.length - 1] as HTMLElement;
        const lastCardRightEdge = lastCard.offsetLeft + lastCard.offsetWidth + 24;
        canScrollRight.value = Math.ceil(scrollLeft + clientWidth) < lastCardRightEdge;
    } else {
        canScrollRight.value = false;
    }
};

const scrollToLeft = () => {
    if (activities.value.length > 0) {
        activateAndCenter(activities.value[0].id);
    }
};

const scrollToRight = () => {
    if (activities.value.length > 0) {
        activateAndCenter(activities.value[activities.value.length - 1].id);
    }
};

const activateAndCenter = (id: string) => {
    activeId.value = id;

    const container = trackRef.value;
    if (!container) return;

    const index = activities.value.findIndex(item => item.id === id);
    if (index === -1) return;

    const finalOffsetLeft = 4 + (index * 220);
    const finalActiveWidth = 320;

    let targetScrollLeft = finalOffsetLeft - (container.clientWidth / 2) + (finalActiveWidth / 2);
    targetScrollLeft = Math.max(0, targetScrollLeft);

    container.scrollTo({ left: targetScrollLeft, behavior: 'smooth' });
};

const handleCardClick = (id: string) => {
    activateAndCenter(id);
};

onMounted(async () => {
    await listen<{ id: string, enabled: boolean, isReset?: boolean }>('live-activity-toggle', (event) => {
        if (event.payload.id === 'pomodoro' && event.payload.isReset) {
            const pomoItem = activities.value.find(a => a.id === 'pomodoro');
            if (pomoItem) {
                pomoItem.enabled = false;
            }

            isStarted.value = false;
            localStorage.setItem(NSD_POMODORO_ACTIVE, 'false');
            localStorage.setItem(NSD_POMODORO_STARTED, 'false');
            localStorage.setItem(NSD_POMODORO_VISIBLE, 'false');

            pomodoroTime.value = Number(localStorage.getItem(NSD_POMODORO_TIME)) || 1500;
        }
    });

    nextTick(() => { checkScroll(); });
    window.addEventListener('resize', checkScroll);
});

onUnmounted(() => {
    window.removeEventListener('resize', checkScroll);
});
</script>

<style scoped>
/* 框架布局*/
.gallery-layout-wrapper {
    display: flex;
    flex-direction: column;
    width: 100%;
    flex: 1;
    min-height: 0;
    overflow: hidden;
    position: relative;
}

.gallery-header {
    flex-shrink: 0;
    margin-bottom: 24px;
}

/* 标题区域 — 艺术字排版 */
.title-wrapper {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-left: 4px;
}

.art-title {
    font-size: 60px;
    font-weight: 900;
    letter-spacing: -3px;
    line-height: 1;
    margin: 0;
    color: var(--h1-color, #1a1a1a);
    display: flex;
    align-items: baseline;
    user-select: none;
}

.art-accent {
    -webkit-text-fill-color: transparent;
    -webkit-text-stroke: 1.5px var(--h1-color, #1a1a1a);
    background: none;
}

.art-dot {
    color: #ef4444;
    font-size: 64px;
    line-height: 0.8;
    margin-left: 1px;
}

.title-sub-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 2px;
    flex-wrap: wrap;
}

.title-badge {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 1.5px;
    color: var(--h1-color, #333);
    background: var(--control-bg, rgba(0, 0, 0, 0.04));
    padding: 4px 14px;
    border-radius: 100px;
    border: 1px solid var(--control-border, rgba(0, 0, 0, 0.06));
}

.subtitle-divider {
    font-size: 11px;
    color: var(--control-border, #ddd);
    font-weight: 300;
}

.subtitle-text {
    font-size: 12px;
    font-weight: 600;
    color: var(--subtitle-color, #888);
    letter-spacing: 0.3px;
}

/* 横向控制按钮 */
.scroll-btn {
    position: absolute;
    top: 280px;
    transform: translateY(-50%);
    z-index: 10;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: var(--card-bg, #ffffff);
    border: 1px solid var(--control-border);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--item-title-color);
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.scroll-btn svg {
    width: 20px;
    height: 20px;
}

.scroll-btn:hover {
    transform: translateY(-50%) scale(1.1);
    border-color: var(--accent-color, #ccc);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.12);
}

.scroll-btn-left {
    left: 4px;
}

.scroll-btn-right {
    right: 4px;
}

/* 横向容器 */
.gallery-track {
    display: flex;
    gap: 20px;
    overflow-x: auto;
    padding: 10px 50vw 30px 4px;
    align-items: stretch;
    flex-grow: 1;
    min-height: 0;
    scroll-behavior: smooth;
    scrollbar-width: none;
    -ms-overflow-style: none;
}

.gallery-track::-webkit-scrollbar {
    display: none;
}

@property --mask-left-size {
    syntax: '<length>';
    initial-value: 0px;
    inherits: false;
}

@property --mask-right-size {
    syntax: '<length>';
    initial-value: 0px;
    inherits: false;
}

.gallery-track {
    --mask-left-size: 0px;
    --mask-right-size: 0px;

    -webkit-mask-image: linear-gradient(to right,
            transparent 0%,
            #000 var(--mask-left-size),
            #000 calc(100% - var(--mask-right-size)),
            transparent 100%);
    mask-image: linear-gradient(to right,
            transparent 0%,
            #000 var(--mask-left-size),
            #000 calc(100% - var(--mask-right-size)),
            transparent 100%);

    transition: --mask-left-size 0.4s cubic-bezier(0.16, 1, 0.3, 1),
        --mask-right-size 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.gallery-track.mask-left {
    --mask-left-size: 60px;
}

.gallery-track.mask-right {
    --mask-right-size: 60px;
}

.spacer {
    width: 50vw;
    flex-shrink: 0;
}

/* 卡片 */
.gallery-card {
    flex-shrink: 0;
    width: 200px;
    background: var(--card-bg);
    border: 1px solid var(--control-border);
    border-radius: 20px;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    transform: scale(0.96);
    opacity: 0.45;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.02);
    transition: width 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        transform 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        opacity 0.3s ease,
        box-shadow 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        border-color 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}

.gallery-card:hover {
    opacity: 0.85;
}

.gallery-card.is-active {
    width: 320px;
    transform: scale(1);
    opacity: 1;
    cursor: default;
    border-color: var(--accent-color);
    box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.1),
        0 0 0 1px var(--accent-color) inset;
}

:global(.dark-theme) .gallery-card.is-active {
    box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.5),
        0 0 20px -5px var(--accent-color);
    background: linear-gradient(180deg, var(--control-bg) 0%, var(--card-bg) 100%);
}

.card-hero {
    padding: 24px;
    display: flex;
    flex-direction: column;
    height: 140px;
    flex-shrink: 0;
    position: relative;
}

.hero-top-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: auto;
}

.pro-icon {
    width: 36px;
    height: 36px;
    color: var(--item-title-color);
    transition: all 0.4s ease;
}

.is-active .pro-icon {
    color: var(--accent-color);
    transform: scale(1.1);
}

.hero-text h3 {
    margin: 0;
    font-size: 18px;
    font-weight: 800;
    color: var(--h1-color);
    letter-spacing: -0.5px;
    transition: color 0.3s;
}

.hero-text p {
    margin: 6px 0 0 0;
    font-size: 12px;
    font-weight: 600;
    color: var(--subtitle-color);
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.card-body {
    padding: 0 24px 24px 24px;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
}

.body-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
}

.fade-up-enter-active {
    transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
    transition-delay: 0.1s;
}

.fade-up-enter-from {
    opacity: 0;
    transform: translateY(15px);
}

/* Switch */
.custom-switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    flex-shrink: 0;
}

.custom-switch.mini {
    width: 36px;
    height: 20px;
}

.custom-switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.custom-switch .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--control-border, #e2e8f0);
    border-radius: 24px;
    transition: background-color 0.3s ease;
}

.custom-switch .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: #ffffff;
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.custom-switch.mini .slider:before {
    height: 14px;
    width: 14px;
    left: 3px;
    bottom: 3px;
}

.custom-switch input:checked+.slider {
    background-color: var(--accent-color, #10b981);
}

.custom-switch input:checked+.slider:before {
    transform: translateX(20px);
}

.custom-switch.mini input:checked+.slider:before {
    transform: translateX(16px);
}

.custom-switch input:disabled+.slider {
    cursor: not-allowed;
    opacity: 0.6;
}

.custom-switch input:disabled {
    cursor: not-allowed;
}

/* 排版辅助 */
.pro-setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 12px;
    padding: 14px 0;
    border-bottom: 1px dashed var(--control-border);
}

.pro-setting-item:last-child {
    border-bottom: none;
}

.pro-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex-shrink: 0;
    max-width: 100%;
}

.pro-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--item-title-color);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.mt-10 {
    margin-top: 10px;
}

.pro-coming-soon {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    height: 100%;
    padding: 20px 0;
}

.loader-line {
    width: 40px;
    height: 3px;
    background: var(--control-border);
    position: relative;
    overflow: hidden;
    margin-bottom: 12px;
}

.loader-line::after {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
    width: 15px;
    background: var(--item-title-color);
    animation: loader-slide 1.5s infinite ease-in-out;
}

@keyframes loader-slide {
    0% { transform: translateX(-15px); }
    100% { transform: translateX(40px); }
}

.pro-coming-soon p {
    font-size: 11px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: var(--item-desc-color);
    margin: 0;
    letter-spacing: 0.5px;
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

/* 番茄钟时间设置 */
.time-edit-btn {
    background: transparent;
    border: 1px solid var(--control-border);
    color: var(--item-title-color);
    border-radius: 6px;
    padding: 2px 10px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    outline: none;
}

.time-edit-btn:hover {
    background: var(--control-border);
}

.time-edit-btn:disabled {
    cursor: not-allowed;
    opacity: 0.5;
    background: transparent;
    border-color: var(--control-border);
    color: var(--item-desc-color);
}

.time-display {
    font-size: 22px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: var(--item-title-color);
    text-align: center;
    letter-spacing: 2px;
    padding: 4px 0;
}

.time-input-group {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 4px 0;
}

.time-input {
    width: 44px;
    background: rgba(0, 0, 0, 0.03);
    border: 1px solid var(--control-border);
    border-radius: 8px;
    padding: 6px 4px;
    text-align: center;
    font-size: 14px;
    font-weight: bold;
    color: var(--h1-color);
    outline: none;
    transition: all 0.2s ease;
}

.time-input:focus {
    border-color: var(--accent-color);
    background: transparent;
}

:global(.dark-theme) .time-input {
    background: rgba(255, 255, 255, 0.05);
}

.time-separator {
    font-weight: bold;
    color: var(--item-title-color);
}

.time-input::-webkit-outer-spin-button,
.time-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

.time-input[type=number] {
    -moz-appearance: textfield;
    appearance: textfield;
}

/* 番茄钟按钮 */
.pomodoro-controls {
    display: flex;
    gap: 6px;
    align-items: center;
}

.pomo-btn {
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 700;
    border-radius: 6px;
    cursor: pointer;
    border: 1px solid var(--control-border);
    background: var(--card-bg);
    color: var(--item-title-color);
    transition: all 0.2s ease;
    outline: none;
}

.pomo-btn:hover {
    background: var(--control-border);
}

.toggle-btn:not(.is-running) {
    background: var(--accent-color, #ff4757);
    border-color: var(--accent-color, #ff4757);
    color: #fff;
}

.toggle-btn:not(.is-running):hover {
    opacity: 0.9;
}

.toggle-btn.is-running {
    background: var(--card-bg);
    border-color: var(--control-border);
    color: var(--item-title-color);
}
</style>
