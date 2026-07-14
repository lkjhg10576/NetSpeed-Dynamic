<template>
    <transition @enter="onEnter" @leave="onLeave" :css="false">
        <div v-show="isIslandVisible" :class="['island-container', { 'has-music-border': isGlowBorderEnabled }]"
            @mousedown="handleMouseDown" @mousemove="handleMouseMove" @mouseup="handleMouseUp"
            @mouseleave="handleMouseLeave" @mouseenter="handleMouseEnter" :style="islandStyle"
            @contextmenu="handleRightClick">

            <div class="rainbow-border-glow" v-if="isGlowBorderEnabled" :style="{ opacity: glowOpacity }"></div>

            <!-- 宸︿晶瀹藉害璋冩暣鎵嬫焺 -->
            <div class="resize-handle left"
                v-if="!isPositionLocked && !isMusicExpanded && !isMusicExpanding && !isMsgActive && !displaySysToast"
                @mousedown.stop="handleResizeStart($event, 'left')">
            </div>

            <div class="island-core-content" :style="coreContentStyle" :class="{ 'resize-cursor-left': mouseNearEdge === 'left', 'resize-cursor-right': mouseNearEdge === 'right' }">
                <div class="inner-wrapper">
                    <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div v-if="isMsgActive" class="msg-box" key="msg" @click="handleNotificationClick"
                            style="cursor: pointer;">
                            <div class="msg-avatar">
                                <img :src="currentMsgIcon" alt="娑堟伅鍥炬爣" class="msg-avatar-img">
                            </div>
                            <div class="msg-text-wrapper">
                                <div class="msg-title">
                                    <span class="sender-name">{{ msgTitle }}</span>
                                    <span class="app-name">{{ msgAppName }}</span>
                                </div>
                                <div class="msg-body">{{ msgBody }}</div>
                            </div>
                        </div>

                        <div v-else-if="displaySysToast" class="system-toast-box" key="systoast">
                            <div v-if="sysToastType === 'app'" class="toast-icon app-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <circle cx="12" cy="12" r="10" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" opacity="0.3" />
                                    <path d="M8 12.5l3 3 5-6" stroke-width="2.5" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'lock'" class="toast-icon sys-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="4" y="12" width="16" height="8" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <path d="M8 12V9a4 4 0 0 1 8 0v3" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'unlock'" class="toast-icon sys-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="4" y="12" width="16" height="8" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <path d="M8 12V9a4 4 0 0 1 8 0" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'battery-charge'" class="toast-icon battery-charge-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="2" y="7" width="16" height="10" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <line x1="22" y1="11" x2="22" y2="13" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                    <polygon points="11 7 8 12 12 12 11 17 14 12 10 12 11 7" stroke-width="1.5"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'battery-low'" class="toast-icon battery-low-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="2" y="7" width="16" height="10" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <line x1="22" y1="11" x2="22" y2="13" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                    <line x1="6" y1="12" x2="9" y2="12" stroke-width="4" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else class="toast-icon sys-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <circle cx="12" cy="12" r="10" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" opacity="0.3" />
                                    <g transform="translate(6, 5.5) scale(0.5)">
                                        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" stroke-width="4"
                                            stroke-linecap="round" stroke-linejoin="round" />
                                        <path d="M13.73 21a2 2 0 0 1-3.46 0" stroke-width="4" stroke-linecap="round"
                                            stroke-linejoin="round" />
                                    </g>
                                </svg>
                            </div>
                            <div class="toast-text">{{ sysToastText }}</div>
                        </div>

                        <div v-else-if="displayHardware" class="speed-box" key="hardware">
                            <transition name="speed-fade" mode="out-in">
                                <div v-if="isShowingCPU" class="speed-item" key="cpu">
                                    <span class="label">CPU</span>
                                    <span class="value" :class="{ 'high-usage': parseInt(cpuUsage) >= 90 }">{{ cpuUsage
                                        }}</span>
                                </div>
                                <div v-else class="speed-item" key="ram">
                                    <span class="label">RAM</span>
                                    <span class="value" :class="{ 'high-usage': parseInt(memUsage) >= 90 }">{{ memUsage
                                        }}</span>
                                </div>
                            </transition>
                        </div>

                        <div v-else-if="displayMusic" class="music-ctl-box" :class="{ 'expanded': isMusicExpanded }"
                            :key="'music_' + musicBoxKey" @click="expandMusic" style="cursor: pointer;">
                            <div class="music-top-row">
                                <div class="album-cover" :class="{ 'is-playing': isPlaying }">
                                    <div class="cover-inner"
                                        :style="coverUrl ? { backgroundImage: `url(${coverUrl})`, backgroundSize: 'cover' } : {}">
                                    </div>
                                </div>
                                <div class="music-info-mask-box" ref="maskBoxRef">
                                    <div class="music-info-text single-line" :class="{ 'fade-out': isMusicExpanded }">
                                        <span class="scroll-inner" ref="textInnerRef"
                                            :class="{ 'is-scrolling': scrollDist > 0 }"
                                            :style="scrollDist > 0 ? { '--scroll-dist': scrollDist + 'px', '--scroll-duration': scrollDuration } : {}">
                                            {{ currentTrackInfo }}
                                        </span>
                                    </div>
                                    <div class="music-info-text double-line" :class="{ 'fade-in': isMusicExpanded }">
                                        <div class="song-title">{{ currentSongName }}</div>
                                        <div class="song-artist">{{ currentArtistName }}</div>
                                    </div>
                                </div>
                            </div>
                            <transition name="fade">
                                <div class="music-controls" v-show="isMusicExpanded">
                                    <button class="ctl-btn" @click.stop="prevTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
                                        </svg>
                                    </button>
                                    <button class="ctl-btn play-btn" @click.stop="togglePlay">
                                        <svg v-if="isPlaying" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
                                        </svg>
                                        <svg v-else viewBox="0 0 24 24" fill="currentColor"
                                            style="transform: translateX(1px);">
                                            <path d="M8 5v14l11-7z" />
                                        </svg>
                                    </button>
                                    <button class="ctl-btn" @click.stop="nextTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
                                        </svg>
                                    </button>
                                </div>
                            </transition>
                            <transition name="fade">
                                <div class="music-progress" v-show="isMusicExpanded" @click.stop>
                                    <template v-if="timelineStatus === 'available'">
                                        <div class="progress-time-row">
                                            <span class="progress-time">{{ formatTime(progressPosition) }}</span>
                                            <span class="progress-time">{{ formatTime(progressEnd) }}</span>
                                        </div>
                                        <div class="progress-bar" ref="progressBarRef"
                                            :class="{ disabled: !musicTimeline.canSeek }"
                                            :aria-disabled="!musicTimeline.canSeek"
                                            @pointerdown.stop="onProgressPointerDown"
                                            @pointermove="onProgressPointerMove"
                                            @pointerup="onProgressPointerUp"
                                            @pointercancel="onProgressPointerCancel"
                                            @lostpointercapture="onProgressPointerCancel">
                                            <div class="progress-filled" :class="{ dragging: isDraggingProgress }"
                                                :style="{ width: progressPercent + '%' }"></div>
                                            <div class="progress-thumb" :style="{ left: progressPercent + '%' }"></div>
                                        </div>
                                        <div class="progress-remaining">-{{ formatTime(progressEnd - progressPosition) }}</div>
                                    </template>
                                    <div v-else class="progress-placeholder">
                                        {{ timelineStatus === 'loading' ? '姝ｅ湪璇诲彇鎾斁杩涘害鈥? : '褰撳墠鎾斁鍣ㄦ湭鎻愪緵鎾斁杩涘害' }}
                                    </div>
                                </div>
                            </transition>
                        </div>

                        <div v-else-if="displaySpeed" class="speed-box" key="speed">
                            <transition name="speed-fade" mode="out-in">
                                <div v-if="isShowingUpload" class="speed-item" key="upload">
                                    <span :class="['label', { 'high-traffic': isHighUpload }]">猬?/span>
                                    <span class="value">{{ uploadSpeed }}</span>
                                </div>
                                <div v-else class="speed-item" key="download">
                                    <span :class="['label', { 'high-traffic': isHighDownload }]">猬?/span>
                                    <span class="value">{{ downloadSpeed }}</span>
                                </div>
                            </transition>
                        </div>
                    </transition>
                </div>

                <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                    <div v-if="showSpectrumIndicator" class="audio-spectrum"
                        :class="{ 'is-playing': isPlaying, 'expanded': isMusicExpanded }" key="spectrum">
                        <span class="bar" v-for="(val, index) in spectrumData" :key="index"
                            :style="{ transform: `scaleY(${val})` }"></span>
                    </div>

                    <div v-else :class="['status-dot', networkStatus]" key="dot"></div>
                </transition>
            </div>

            <!-- 鍙充晶瀹藉害璋冩暣鎵嬫焺 -->
            <div class="resize-handle right"
                v-if="!isPositionLocked && !isMusicExpanded && !isMusicExpanding && !isMsgActive && !displaySysToast"
                @mousedown.stop="handleResizeStart($event, 'right')">
            </div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch, nextTick, type CSSProperties } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition, LogicalPosition, PhysicalSize } from '@tauri-apps/api/window'; import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';
import { formatSpeed } from '../utils/format';
import {
    NSD_AUTO_HIDE_DELAY, NSD_AUTO_HIDE_ENABLED,
    NSD_AUTO_COLLAPSE_DELAY, NSD_AUTO_COLLAPSE_ENABLED,
    NSD_ISLAND_OPACITY, NSD_ISLAND_THEME,
    NSD_HARDWARE_MON, NSD_MUSIC_CTRL, NSD_GLOW_BORDER,
    NSD_PIN_TASKBAR, NSD_POSITION_LOCKED,
    NSD_MSG_MODE, NSD_ROTATION_MODE,
    NSD_ISLAND_WIDTH, NSD_ISLAND_POSITION, NSD_MSG_NOTIFY,
    NSD_TARGET_PLAYER
} from '../constants/storageKeys';

const isIslandVisible = ref(false);
const isMenuOpen = ref(false);

// 鑷姩闅愯棌鐩稿叧鍙橀噺
const isMouseOver = ref(false);
let autoHideTimer: number | null = null;
const autoHideDelay = ref(Number(localStorage.getItem(NSD_AUTO_HIDE_DELAY) || '2000')); // 榛樿2绉?
const isAutoHideEnabled = ref(localStorage.getItem(NSD_AUTO_HIDE_ENABLED) === 'true'); // 鑷姩闅愯棌鍔熻兘寮€鍏?

// 鑷姩鎶樺彔鐩稿叧鍙橀噺锛堢伒鍔ㄥ矝灞曞紑鍚庯紝榧犳爣绂诲紑鑷姩鎶樺彔鍥炲皬宀涚姸鎬侊級
let autoCollapseTimer: number | null = null;
const autoCollapseDelay = ref(Number(localStorage.getItem(NSD_AUTO_COLLAPSE_DELAY) || '2000')); // 榛樿2绉?
const isAutoCollapseEnabled = ref(localStorage.getItem(NSD_AUTO_COLLAPSE_ENABLED) === 'true'); // 鑷姩鎶樺彔鍔熻兘寮€鍏?

// 璁板綍褰撳墠鏄惁鏄剧ず涓婅缃戦€燂紙鐢ㄤ簬杞崲锛?
const isShowingUpload = ref(false);
const isShowingCPU = ref(true);
let speedCycleTimer: number | null = null;

// 鎺у埗 DOM 鐪熸鐨勯珮瀹藉彉閲忎笌娑堟伅鏁版嵁
const currentWidth = ref(150);
const currentHeight = ref(34);
const isMsgActive = ref(false);
const msgTitle = ref('');
const msgAppName = ref('');
const msgBody = ref('');
const msgAumid = ref('');

// 绯荤粺鎿嶄綔閫氱煡涓撶敤鍙橀噺
const displaySysToast = ref(false);
const sysToastText = ref('');
const sysToastType = ref<'app' | 'sys' | 'battery-charge' | 'battery-low' | 'lock' | 'unlock'>('app');
const toastQueue = ref<{ text: string, type: 'app' | 'sys' | 'battery-charge' | 'battery-low' | 'lock' | 'unlock' }[]>([]);
let isProcessingToast = false;

// 闃熷垪澶勭悊鍑芥暟
const processToastQueue = async () => {
    // 濡傛灉姝ｅ湪澶勭悊锛屾垨鑰呴槦鍒椾负绌猴紝鍒欑洿鎺ヨ繑鍥?
    if (isProcessingToast || toastQueue.value.length === 0) return;

    // 浼樺厛绾у垽鏂細濡傛灉褰撳墠姝ｅ湪鏄剧ず娑堟伅閫氱煡(鏈€楂樹紭鍏堢骇)锛屽垯鎸傝捣绛夊緟
    if (isMsgActive.value) return;

    isProcessingToast = true;
    const nextToast = toastQueue.value.shift();

    if (nextToast) {
        sysToastText.value = nextToast.text;
        sysToastType.value = nextToast.type;
        displaySysToast.value = true;
        
        // 鑷姩鎭㈠鏄剧ず锛氬綋鏈夌郴缁熼€氱煡鏃讹紝濡傛灉鐏靛姩宀涜闅愯棌锛屽垯鑷姩鎭㈠鏄剧ず
        if (!isIslandVisible.value) {
            getCurrentWindow().show();
            isIslandVisible.value = true;
        }

        // 鍋滅暀鏄剧ず
        await new Promise(resolve => setTimeout(resolve, 2000));

        displaySysToast.value = false;
        // 绛夊緟绂诲紑鍔ㄧ敾鎾畬 (绾?200ms) 鍐嶅鐞嗕笅涓€涓?
        await new Promise(resolve => setTimeout(resolve, 200));
    }

    isProcessingToast = false;
    processToastQueue(); // 閫掑綊妫€鏌ユ槸鍚﹁繕鏈変笅涓€涓€氱煡
};

// 鐩戝惉绯荤粺閫氱煡鏄剧ず鐘舵€侊紝瑙ｅ喅缃戦€熸ā寮忎笅灏哄杩囧皬瀵艰嚧鏂囧瓧婧㈠嚭/閬尅鎸囩ず鐏殑闂
watch(displaySysToast, (newVal) => {
    if (newVal) {
        // 褰撴湁绯荤粺鎿嶄綔閫氱煡鍑虹幇鏃讹紝寮哄埗灞曞紑鍒伴粯璁ゆ爣鍑嗗昂瀵?
        animateIslandSize(260, 42);
    } else {
        // 閫氱煡娑堝け鏃讹紝鎭㈠鍒板綋鍓嶇姸鎬佽鏈夌殑灏哄
        // 锛堝墠鎻愭槸娌℃湁琚簲鐢ㄦ秷鎭垨闊充箰闈㈡澘闇稿崰锛?
        if (!isMsgActive.value && !isMusicExpanded.value && !isMusicExpanding.value) {
            const { h } = getBaseSize();
            const savedWidth = restoreIslandWidth();
            const targetWidth = savedWidth !== null ? savedWidth : currentWidth.value;
            animateIslandSize(targetWidth, h);
        }
    }
});

// 鏆撮湶缁欏閮ㄨ皟鐢ㄧ殑瑙﹀彂鍑芥暟
const showToast = (text: string, type: 'app' | 'sys' | 'battery-charge' | 'battery-low' | 'lock' | 'unlock' = 'app') => {
    toastQueue.value.push({ text, type });
    processToastQueue();
};

// 鐩戝惉娑堟伅閫氱煡鐘舵€侊紝涓€鏃︽秷鎭€氱煡娑堝け锛岀珛鍒诲敜閱掑彲鑳借鎸傝捣鐨勬搷浣滈€氱煡闃熷垪
watch(isMsgActive, (newVal) => {
    if (!newVal) {
        processToastQueue();
    }
});

// 璁板綍闊充箰宀涙槸鍚﹀浜庡睍寮€鐘舵€?
const isMusicExpanded = ref(false);
const isMusicExpanding = ref(false); // 璁板綍鏄惁姝ｅ湪鎾斁寮规€ф寜鍘嬪睍寮€鍔ㄧ敾
let musicExpandAnimTimer: number | null = null; // 鐢ㄤ簬鎺ョ灞曞紑鏃剁殑瀹氭椂鍣紝闃叉鍐茬獊

// 鐏靛姩宀涜嚜韬殑閫忔槑搴﹀彉閲忥紙榛樿100锛?
const islandOpacity = ref(Number(localStorage.getItem(NSD_ISLAND_OPACITY) || '100'));

// 鐏靛姩宀涜嚜韬富棰樿壊
const islandTheme = ref(localStorage.getItem(NSD_ISLAND_THEME) || 'black');

// 1. 鐬棿鍒ゅ畾褰撳墠鏄惁澶勪簬澶х獥鍙ｇ姸鎬?
const isExpandedSize = computed(() => isMusicExpanded.value || isMsgActive.value);

// 2. 澶栧眰瀹瑰櫒锛氱姸鎬佷竴鍙橈紝绔嬮┈鍒囨垚鐩爣鍦嗚
const islandStyle = computed<CSSProperties>(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);
    const baseStyle = islandTheme.value === 'white' ? {
        backgroundColor: `rgba(255, 255, 255, ${alpha})`,
        color: '#000000'
    } : {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`,
        color: '#ffffff'
    };

    return {
        ...baseStyle,
        width: '100vw',
        height: '100vh',
        // 鍙灞曞紑灏辨槸 24px锛屾敹璧峰氨鏄?100px
        borderRadius: isExpandedSize.value ? '24px' : '100px',
        position: 'relative',
    };
});

// 3. 鍐呭眰鏍稿績锛氭案杩滄瘮澶栧眰灏?2px
const coreContentStyle = computed(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);

    // 灞曞紑 22px锛屾敹璧?98px
    const innerRadius = isExpandedSize.value ? '22px' : '98px';

    if (islandTheme.value === 'white') {
        return {
            backgroundColor: `rgba(255, 255, 255, ${alpha})`,
            borderRadius: innerRadius
        };
    }
    return {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`,
        borderRadius: innerRadius
    };
});

const glowOpacity = computed(() => {
    const linear = islandOpacity.value / 100;
    return Math.pow(linear, 1 / 2.2);
});

const uploadSpeed = ref('0 KB/s');
const downloadSpeed = ref('0 KB/s');

// 璁板綍褰撳墠鏄惁灞炰簬澶ф祦閲忕姸鎬?
const isHighDownload = ref(false);
const isHighUpload = ref(false);

// 缃戠粶鐘舵€佹寚绀虹伅锛歡ood(缁?, warning(榛?, error(绾?
const networkStatus = ref<'good' | 'warning' | 'error'>('good');

// 绯荤粺纭欢鐩戞帶鐩稿叧
const isHardwareMonEnabled = ref(localStorage.getItem(NSD_HARDWARE_MON) === 'true');
const cpuUsage = ref('0%');
const memUsage = ref('0%');

// 闊充箰鎺у埗鍔熻兘寮€鍏?
const isMusicCtlEnabled = ref(localStorage.getItem(NSD_MUSIC_CTRL) === 'true');
const isPlaying = ref(false);
// 娴佸厜杈规榛樿鐘舵€佸畬鍏ㄩ暅鍍忛煶涔愭帶鍒跺櫒锛堝彧瑕侀煶涔愭帶鍒跺櫒寮€鐫€瀹冨氨寮€锛屽叧浜嗗氨涓€璧峰叧锛?
const isGlowBorderEnabled = ref(localStorage.getItem(NSD_GLOW_BORDER) === 'true');

// 寰嬪姩棰戣氨
const spectrumData = ref([0.35, 0.35, 0.35, 0.35, 0.35]);

// 灏侀潰url
const coverUrl = ref('');
const coverCache = new Map<string, string>();

// 璁板綍鏄惁寮€鍚簡缃簬浠诲姟鏍?
const isPinnedToTaskbar = ref(localStorage.getItem(NSD_PIN_TASKBAR) === 'true');
// 璁板綍鏄惁閿佸畾浜嗕綅缃紝骞跺瓨鍒版湰鍦?
const isPositionLocked = ref(localStorage.getItem(NSD_POSITION_LOCKED) === 'true');

// 瀹藉害璋冩暣鐩稿叧鐘舵€?
const isResizing = ref(false);
const resizeSide = ref<'left' | 'right' | null>(null);
let resizeStartX = 0;
let resizeStartWidth = 0;
const MIN_WIDTH = 100; // 鏈€灏忓搴?
const MAX_WIDTH = 500; // 鏈€澶у搴?

// 榧犳爣鏄惁鍦ㄨ竟缂樺尯鍩燂紙鐢ㄤ簬鍏夋爣鏍峰紡锛?
const mouseNearEdge = ref<'left' | 'right' | null>(null);

// 璁＄畻鏄惁鍙互璋冩暣瀹藉害
const canResize = computed(() => {
    return !isPositionLocked.value && !isMusicExpanded.value && !isMusicExpanding.value && !isMsgActive.value && !displaySysToast.value;
});
// 璁板綍娑堟伅妯″紡寮€鍏崇姸鎬?
const isMsgModeEnabled = ref(localStorage.getItem(NSD_MSG_MODE) === 'true');
// 杞崲鍔熻兘鏍稿績閫昏緫
const isRotationEnabled = ref(localStorage.getItem(NSD_ROTATION_MODE) === 'true');
const currentRotIndex = ref(0); // 0=缃戦€? 1=闊充箰, 2=纭欢
let rotationTimer: number | null = null;

// 浣跨敤璁＄畻灞炴€ф櫤鑳藉垽鏂綋鍓嶈鏄剧ず璋?
const displaySpeed = computed(() => !isMsgActive.value && !displaySysToast.value && (isRotationEnabled.value ? currentRotIndex.value === 0 : (!isMusicCtlEnabled.value && !isHardwareMonEnabled.value)));
const displayMusic = computed(() => !isMsgActive.value && !displaySysToast.value && (isRotationEnabled.value ? currentRotIndex.value === 1 : isMusicCtlEnabled.value));
const displayHardware = computed(() => !isMsgActive.value && !displaySysToast.value && (isRotationEnabled.value ? currentRotIndex.value === 2 : isHardwareMonEnabled.value));

// 杈呭姪鍑芥暟锛氳幏鍙栧綋鍓嶇姸鎬佸簲璇ユ嫢鏈夌殑榛樿澶у皬
const getBaseSize = () => {
    // 缃戦€熷矝 鍜?纭欢鐩戞帶灏哄缁熶竴缂╁皬涓?150x34
    if (displaySpeed.value || displayHardware.value) return { w: 150, h: 34 };
    // 纭欢銆侀煶涔愶紙鏈睍寮€锛夌瓑鍏朵粬鐘舵€佹仮澶嶉粯璁ょ殑 260x42
    return { w: 260, h: 42 };
};

// 鐩戝惉鍐呭鍒囨崲锛岃Е鍙戜笣婊戝姩鐢昏繃娓?
watch([displaySpeed, displayMusic, displayHardware], () => {
    // 鍙湁鍦ㄦ病鏈夎涓存椂寮圭獥锛堟秷鎭€侀煶涔愬睍寮€锛夐湼鍗犳椂锛屾墠鎵ц鍩虹澶у皬鍒囨崲
    if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { h } = getBaseSize();
        const savedWidth = restoreIslandWidth();
        const targetWidth = savedWidth !== null ? savedWidth : currentWidth.value;
        animateIslandSize(targetWidth, h);
    }
});

// 涓撻棬鐢ㄤ簬鎺у埗鍙充晶甯搁┗鎸囩ず鐏殑鐙珛璁＄畻灞炴€э紙瀹屽叏涓嶅彈娑堟伅閫氱煡鎵撴柇锛?
const showSpectrumIndicator = computed(() => {
    return isRotationEnabled.value ? currentRotIndex.value === 1 : isMusicCtlEnabled.value;
});

// 棰戣氨寮€鍏崇姸鎬佽拷韪紝閬垮厤閲嶅璋冪敤鍚庣
let isSpectrumActive = false;

// 鎸夐渶鍚仠闊抽棰戣氨鎹曡幏锛氫粎鍦ㄥ墠绔渶瑕佹樉绀洪璋辨椂鎵嶆縺娲诲悗绔?FFT 杩愮畻锛岀┖闂叉椂闆?CPU 闆跺垎閰?
watch([isPlaying, showSpectrumIndicator], () => {
    const shouldActivate = isPlaying.value && showSpectrumIndicator.value;
    if (shouldActivate && !isSpectrumActive) {
        isSpectrumActive = true;
        invoke('set_spectrum_active', { active: true }).catch(() => {});
    } else if (!shouldActivate && isSpectrumActive) {
        isSpectrumActive = false;
        invoke('set_spectrum_active', { active: false }).catch(() => {});
    }
});

const startRotation = () => {
    if (rotationTimer) clearInterval(rotationTimer);
    rotationTimer = window.setInterval(() => {
        currentRotIndex.value = (currentRotIndex.value + 1) % 3;
    }, 5000); // 5绉掕疆鎹竴娆?
};

// 缁熶竴鐨勮嚜鍔ㄩ殣钘忓畾鏃跺櫒绠＄悊鍑芥暟
const scheduleAutoHide = (delay?: number) => {
    if (autoHideTimer) {
        clearTimeout(autoHideTimer);
        autoHideTimer = null;
    }
    autoHideTimer = window.setTimeout(() => {
        if (!isMouseOver.value && isIslandVisible.value) {
            isAutoHiding = true;
            isIslandVisible.value = false;
        }
    }, delay ?? autoHideDelay.value);
};

const stopRotation = () => {
    if (rotationTimer) {
        clearInterval(rotationTimer);
        rotationTimer = null;
    }
};

// 璁＄畻骞跺惛闄勫埌宸︿笅瑙掔殑鏂规硶
const snapToBottomLeft = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 150));
        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = window.devicePixelRatio;

            const WINDOW_INIT_WIDTH = currentWidth.value;
            const WINDOW_INIT_HEIGHT = currentHeight.value;
            await appWindow.setSize(new PhysicalSize(Math.ceil(WINDOW_INIT_WIDTH * scaleFactor), Math.ceil(WINDOW_INIT_HEIGHT * scaleFactor)));

            const monitorLeftPhysical = monitor.position.x;
            const monitorTopPhysical = monitor.position.y;
            // 鎭㈠浣跨敤 Tauri 鏈€搴曞眰鐨勭‖浠剁湡瀹炲垎杈ㄧ巼锛堢粷瀵逛笉浼氱缉姘达級
            const monitorHeightPhysical = monitor.size.height;

            // X鍧愭爣: 灞忓箷鏈€宸︿晶 + 10px鐨勮竟璺?
            const x = monitorLeftPhysical + (10 * scaleFactor);
            // Y鍧愭爣: 鐗╃悊鏈€搴曢儴 - 绐楀彛楂樺害 - 3px寰皟
            const y = monitorTopPhysical + monitorHeightPhysical - ((WINDOW_INIT_HEIGHT + 3) * scaleFactor);

            // 銆愮粓鏋佺粷鏉€鏍稿績銆戯細缁曡繃 Windows 绯荤粺鐨勪换鍔℃爮闃查伄鎸℃満鍒?
            // 鍦ㄥ己鍒惰鐩栦换鍔℃爮鍧愭爣涔嬪墠锛屽厛闅愯韩锛?
            await appWindow.hide();

            await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));

            // 绉诲姩瀹屾垚鍚庯紝鐬棿鐜拌韩锛岀敓绫崇叜鎴愮啛楗紝Windows 涔熸嫤涓嶄綇浜嗭紒
            await appWindow.show();
        }
    } catch (error) {
        console.error('鍋滈潬宸︿笅瑙掑け璐?', error);
    }
};

const togglePlay = async () => {
    // 1. 鍓嶇鍏堢珛鍒诲垏鎹㈠浘鏍囷紝缁欑敤鎴锋瀬閫熺殑瑙嗚鍙嶉
    isPlaying.value = !isPlaying.value;

    // 2. 鍙戦€佹寚浠ょ粰 Rust 鍜?SMTC
    try {
        await invoke('control_system_media', { action: 'play_pause' });
    } catch (err) {
        console.error('鎾斁鎺у埗澶辫触:', err);
        // 濡傛灉搴曞眰鎺у埗澶辫触浜嗭紝鍐嶆妸鍥炬爣鐘舵€佸洖婊氬洖鏉?
        isPlaying.value = !isPlaying.value;
    }
};

const prevTrack = async () => {
    await invoke('control_system_media', { action: 'prev' });
};

const nextTrack = async () => {
    await invoke('control_system_media', { action: 'next' });
};

// 鏍稿績鍚屾鍑芥暟锛氬鍏ュ埌浣犵殑 fetchSpeedStats 鍚屼竴棰戞鐨勫畾鏃跺櫒涓?
const syncMusicStatus = async () => {
    try {
        // 1. 璋冪敤 Rust 鎻愬彇缃戞槗浜戞爣棰?[姝屽悕, 姝屾墜, 鏄惁鍦ㄦ挱鏀綸
        const res = await invoke<[string, string, boolean] | null>('fetch_netease_music_info');

        if (res) {
            const [song, artist, playing] = res;

            // 鏂板杩欎袱琛屼负浜嗗睍寮€鍚庣殑鍙岃鏄剧ず鍒嗗埆璧嬪€?
            currentSongName.value = song;
            currentArtistName.value = artist || '鏈煡姝屾墜';

            // 鎷兼帴鏂扮殑姝屾洸淇℃伅
            const newTrackInfo = artist ? `${song} - ${artist}` : song;

            if (currentTrackInfo.value !== newTrackInfo) {
                currentTrackInfo.value = newTrackInfo;

                // 浼樺厛璇诲彇缂撳瓨锛圠RU锛氬懡涓椂鍒锋柊鎻掑叆椤哄簭锛岃秴闄愭椂娣樻卑鏈€鏃ф潯鐩級
                if (coverCache.has(newTrackInfo)) {
                    // 鍛戒腑锛氬厛鍒犲啀璁撅紝灏嗚鏉＄洰绉诲埌 Map 鏈熬锛堟渶鏂帮級
                    const cached = coverCache.get(newTrackInfo)!;
                    coverCache.delete(newTrackInfo);
                    coverCache.set(newTrackInfo, cached);
                    coverUrl.value = cached;
                } else {
                    try {
                        const realCoverUrl = await invoke<string>('get_random_cover_url', {
                            songName: song,
                            artistName: artist
                        });
                        coverUrl.value = realCoverUrl;
                        // 鍐欏叆缂撳瓨锛岃秴闄愰€愭潯娣樻卑鏈€鏃ф潯鐩紙LRU锛?
                        while (coverCache.size >= 50) {
                            const oldest = coverCache.keys().next().value;
                            if (oldest !== undefined) coverCache.delete(oldest);
                        }
                        coverCache.set(newTrackInfo, realCoverUrl);
                    } catch (coverErr) {
                        console.error('鎵€鏈夊皝闈㈡簮鍧囪幏鍙栧け璐?', coverErr);
                        // 浣跨敤鏈湴鍥炬爣鎴栫函鑹茶儗鏅紝涓嶈鍐嶇敤澶栭儴 URL 浣滀负閿欒鍏滃簳
                        coverUrl.value = '';
                    }
                }
            }

            const wasPlaying = isPlaying.value;
            isPlaying.value = playing;
            
            // 闊充箰鎾斁鍣ㄦā寮忥細鏈夐煶涔愬氨鏄剧ず锛屾病闊充箰灏遍殣钘忥紙绫讳技閫氱煡妯″紡锛?
            if (displayMusic.value) {
                if (playing && !isIslandVisible.value) {
                    // 鏈夐煶涔愭挱鏀句笖鐏靛姩宀涜闅愯棌锛岃嚜鍔ㄦ仮澶嶆樉绀?
                    getCurrentWindow().show();
                    isIslandVisible.value = true;
                } else if (!playing && wasPlaying && isIslandVisible.value && !isMouseOver.value) {
                    // 闊充箰鍋滄鎾斁涓旈紶鏍囦笉鍦ㄧ伒鍔ㄥ矝涓婏紝寤惰繜闅愯棌
                    scheduleAutoHide();
                }
            }
        } else {
            // 娌℃娴嬪埌鎾斁鏃讹紝娓呯┖鐘舵€?
            currentTrackInfo.value = `鏈湪鎾斁姝屾洸 - ${getPlayerName()}`;
            const wasPlaying = isPlaying.value;
            isPlaying.value = false;
            coverUrl.value = ''; // 娌℃瓕鏃舵竻绌猴紝鏄剧ず榛樿鐨勪紭缇庢笎鍙樿壊

            // 闊充箰鎾斁鍣ㄦā寮忥細闊充箰鍋滄鏃堕殣钘忕伒鍔ㄥ矝
            if (displayMusic.value && wasPlaying && isIslandVisible.value && !isMouseOver.value) {
                scheduleAutoHide();
            }
        }
    } catch (err) {
        console.error('闊充箰淇℃伅鑾峰彇澶辫触:', err);
    }
};

const showInfo = ref(false);
// 榛樿鏄剧ず鍐呭鍔ㄦ€佷粠鏈湴缂撳瓨璇诲彇
const getPlayerName = () => {
    const key = localStorage.getItem(NSD_TARGET_PLAYER) || 'netease';
    const map: Record<string, string> = { 
        'netease': '缃戞槗浜戦煶涔?, 
        'spotify': 'Spotify', 
        'apple': 'Apple Music', 
        'qqmusic': 'QQ闊充箰', 
        'kugou': '閰风嫍闊充箰', 
        'echo': 'Echo Music',
        'smtc': 'SMTC',  // Windows鍘熺敓濯掍綋鎺т欢妯″紡
    };
    return map[key] || '鏈煡骞冲彴';
};

// 瀹氫箟涓€涓敤浜庡己鍒跺埛鏂扮殑 key
const musicBoxKey = ref(0);

// ===== F6 闊充箰杩涘害鏉★細灞曞紑鎬佹媺鍙?SMTC Timeline 骞舵敮鎸佹嫋鍔ㄥ畾浣?=====
type TimelineStatus = 'idle' | 'loading' | 'available' | 'unavailable';
type MusicTimelineResponse = { position_ms: number; end_ms: number; can_seek: boolean };

const musicTimeline = ref({ position: 0, end: 0, canSeek: false });
const timelineStatus = ref<TimelineStatus>('idle');
const timelineClock = ref(Date.now());
const timelineSyncedAt = ref(Date.now());
const isDraggingProgress = ref(false);
const dragPosition = ref(0);
let progressTimer: number | null = null;
let progressClockTimer: number | null = null;
let timelineMissCount = 0;
let isTimelineRequestInFlight = false;
const progressBarRef = ref<HTMLElement | null>(null);

const progressEnd = computed(() => musicTimeline.value.end);
// 鎷栧姩鏃舵樉绀轰复鏃朵綅缃紱鎾斁鏃跺湪涓ゆ SMTC 鍚屾涔嬮棿鎸夋湰鍦版椂閽熷钩婊戞帹杩涖€?
const progressPosition = computed(() => {
    if (isDraggingProgress.value) return dragPosition.value;
    const elapsed = isPlaying.value ? Math.max(0, timelineClock.value - timelineSyncedAt.value) : 0;
    return Math.min(progressEnd.value, musicTimeline.value.position + elapsed);
});
const progressPercent = computed(() => progressEnd.value > 0
    ? Math.min(100, Math.max(0, (progressPosition.value / progressEnd.value) * 100))
    : 0);

// 姣杞?m:ss
const formatTime = (ms: number) => {
    if (ms < 0 || isNaN(ms)) ms = 0;
    const totalSec = Math.floor(ms / 1000);
    const m = Math.floor(totalSec / 60);
    const s = totalSec % 60;
    return `${m}:${s.toString().padStart(2, '0')}`;
};

const resetTimeline = (status: TimelineStatus = 'idle') => {
    isDraggingProgress.value = false;
    dragPosition.value = 0;
    musicTimeline.value = { position: 0, end: 0, canSeek: false };
    timelineStatus.value = status;
    timelineMissCount = 0;
    timelineSyncedAt.value = Date.now();
    timelineClock.value = timelineSyncedAt.value;
};

// 鎷夊彇鎾斁杩涘害锛堜粎灞曞紑鎬佷笖鐏靛姩宀涘彲瑙佹椂鎵ц锛屾姌鍙?闅愯棌鏃舵殏鍋滐級銆?
const fetchTimeline = async () => {
    if (!isMusicExpanded.value || !isIslandVisible.value || isDraggingProgress.value || isTimelineRequestInFlight) return;
    isTimelineRequestInFlight = true;
    try {
        const res = await invoke<MusicTimelineResponse | null>('get_music_timeline');
        if (res && res.end_ms > 0) {
            const now = Date.now();
            musicTimeline.value = {
                position: Math.min(res.position_ms, res.end_ms),
                end: res.end_ms,
                canSeek: res.can_seek,
            };
            timelineSyncedAt.value = now;
            timelineClock.value = now;
            timelineMissCount = 0;
            timelineStatus.value = 'available';
        } else {
            // SMTC 鍋跺皵浼氬湪鍒囨瓕鏃剁煭鏆傝繑鍥炵┖ Timeline锛岃繛缁け璐ュ悗鎵嶅垽瀹氫笉鍙敤銆?
            timelineMissCount++;
            if (timelineMissCount >= 3) timelineStatus.value = 'unavailable';
        }
    } catch (e) {
        timelineMissCount++;
        if (timelineMissCount >= 3) timelineStatus.value = 'unavailable';
    } finally {
        isTimelineRequestInFlight = false;
    }
};

const startProgressTimer = () => {
    stopProgressTimer();
    if (timelineStatus.value !== 'available') timelineStatus.value = 'loading';
    fetchTimeline();
    progressTimer = setInterval(fetchTimeline, 1000) as unknown as number;
    progressClockTimer = setInterval(() => {
        timelineClock.value = Date.now();
    }, 100) as unknown as number;
};

const stopProgressTimer = () => {
    if (progressTimer) {
        clearInterval(progressTimer);
        progressTimer = null;
    }
    if (progressClockTimer) {
        clearInterval(progressClockTimer);
        progressClockTimer = null;
    }
};

// 鎷栧姩瀹氫綅锛氬皢鎸囬拡妯潗鏍囨崲绠椾负鎾斁浣嶇疆
const updateDragPosition = (e: PointerEvent) => {
    if (!progressBarRef.value || progressEnd.value === 0) return;
    const rect = progressBarRef.value.getBoundingClientRect();
    const ratio = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    dragPosition.value = Math.round(ratio * progressEnd.value);
};

const onProgressPointerDown = (e: PointerEvent) => {
    if (progressEnd.value === 0 || !musicTimeline.value.canSeek) return;
    e.preventDefault();
    isDraggingProgress.value = true;
    dragPosition.value = progressPosition.value;
    try { (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId); } catch (_) {}
    updateDragPosition(e);
};

const onProgressPointerMove = (e: PointerEvent) => {
    if (!isDraggingProgress.value) return;
    updateDragPosition(e);
};

const onProgressPointerCancel = (e?: PointerEvent) => {
    if (!isDraggingProgress.value) return;
    isDraggingProgress.value = false;
    dragPosition.value = musicTimeline.value.position;
    if (e) {
        try { (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId); } catch (_) {}
    }
};

const onProgressPointerUp = async (e: PointerEvent) => {
    if (!isDraggingProgress.value) return;
    const finalPos = dragPosition.value;
    const previousPosition = musicTimeline.value.position;
    isDraggingProgress.value = false;
    try { (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId); } catch (_) {}

    const now = Date.now();
    musicTimeline.value = { ...musicTimeline.value, position: finalPos };
    timelineSyncedAt.value = now;
    timelineClock.value = now;
    try {
        await invoke('seek_music', { positionMs: finalPos });
        await fetchTimeline();
    } catch (err) {
        musicTimeline.value = { ...musicTimeline.value, position: previousPosition };
        timelineSyncedAt.value = Date.now();
        timelineClock.value = timelineSyncedAt.value;
        console.error('鎷栧姩瀹氫綅澶辫触:', err);
        await fetchTimeline();
    }
};

// 灞曞紑鎬?+ 鐏靛姩宀涘彲瑙?鏃跺惎鍔ㄨ繘搴︽潯杞锛涙姌鍙?闅愯棌鏃舵殏鍋滃苟娓呯┖鏁版嵁
watch([isMusicExpanded, isIslandVisible], () => {
    if (isMusicExpanded.value && isIslandVisible.value) {
        startProgressTimer();
    } else {
        stopProgressTimer();
        if (!isMusicExpanded.value) {
            // 鎶樺彔鏃舵竻绌猴紝閬垮厤涓嬫灞曞紑鐬棿娈嬬暀鏃ф瓕鏇茶繘搴︺€?
            resetTimeline();
        }
    }
});

// 瀹氫箟鍙岃鏂囨湰鎵€闇€鐨勫崟鐙彉閲?
const currentSongName = ref('鏈湪鎾斁姝屾洸');
const currentArtistName = ref(getPlayerName());
const currentTrackInfo = ref(`鏈湪鎾斁姝屾洸 - ${getPlayerName()}`);

watch(currentTrackInfo, () => {
    if (!isMusicExpanded.value) return;
    resetTimeline('loading');
    fetchTimeline();
});

// 闊充箰婊氬姩鐩稿叧鍙橀噺
const maskBoxRef = ref<HTMLElement | null>(null);
const textInnerRef = ref<HTMLElement | null>(null);
const scrollDist = ref(0);
const scrollDuration = ref('0s');

// 鏍稿績璁＄畻鍑芥暟锛氬垽鏂枃鏈槸鍚﹁秴鍑哄鍣紝骞跺姩鎬佽皟鏁存粴鍔ㄩ€熷害鍜屾椂闀?
const calculateScroll = () => {
    if (!textInnerRef.value || !maskBoxRef.value) return;

    // 灞曞紑鐘舵€佷笅涓嶆墽琛屾粴鍔?
    if (isMusicExpanded.value) {
        scrollDist.value = 0;
        return;
    }

    // 鏍稿績淇 1锛氫娇鐢?getBoundingClientRect() 鑾峰彇鏃犺鐖剁骇闄愬埗鐨勭湡瀹炴覆鏌撳搴?
    const textWidth = textInnerRef.value.getBoundingClientRect().width;
    const containerWidth = maskBoxRef.value.clientWidth;

    if (textWidth > containerWidth) {
        // 鏍稿績淇 1锛氫娇鐢?Math.ceil() 寮哄埗鍙栨暣锛岀粷瀵逛笉鍏佽鍑虹幇灏忔暟鍍忕礌锛?
        scrollDist.value = Math.ceil(textWidth - containerWidth + 20);

        // 鎸夌収 30px/s 鐨勯€熷害闃呰锛岃绠楃函绉诲姩鏃堕棿
        const timeToMove = scrollDist.value / 30;

        // 灏嗛灏惧悇鍋滅暀鐨?1s 宸﹀彸锛堝熀浜?0%鍗犳瘮璁＄畻锛夎瀺鍏ユ€绘椂闀夸腑锛岀‘淇濆寑閫?
        const totalDuration = timeToMove / 0.6;

        scrollDuration.value = `${Math.max(totalDuration, 4.5)}s`;
    } else {
        scrollDist.value = 0;
    }
};

// 鏍稿績淇 2锛氱洃鍚暟缁勫繀椤诲甫涓?displayMusic锛屽苟鍦?nextTick 鍚庡姞涓婂井灏忓欢杩燂紝闃叉 v-else-if 瀵艰嚧瀹藉害鎷垮埌 0
watch([currentTrackInfo, displayMusic, isMusicExpanded], async () => {
    await nextTick();
    setTimeout(() => {
        if (displayMusic.value) {
            calculateScroll();
        } else {
            // 鍒囧埌鍏朵粬鐣岄潰锛堟瘮濡傜綉閫燂級鏃讹紝褰掗浂閲嶇疆
            scrollDist.value = 0;
        }
    }, 100);
});

let lastRx = 0;
let lastTx = 0;
let speedTimer: number;
let pingTimer: number;
let musicTimer: number;
let msgTimer: number | null = null;
let notifyTimer: number;

// 闃叉姈鎺у埗鍙橀噺
let lowTrafficStartTime = Date.now();
const RED_DELAY_MS = 5000;

// 璁＄畻娴侀噺鏁板瓧锛屽苟瀹炴椂鏇存柊澶ф祦閲忕姸鎬?
const fetchSpeedStats = async () => {
    try {
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        if (lastRx !== 0) {
            const rxDiff = currentRx - lastRx;
            const txDiff = currentTx - lastTx;

            downloadSpeed.value = formatSpeed(rxDiff);
            uploadSpeed.value = formatSpeed(txDiff);

            // 1MB = 1048576 瀛楄妭
            const limit = 1024 * 1024;
            const currentDownloadHigh = rxDiff >= limit;
            const currentUploadHigh = txDiff >= limit;

            isHighDownload.value = currentDownloadHigh;
            isHighUpload.value = currentUploadHigh;

            // 缁存姢浣庢祦閲忔寔缁椂闂?
            if (currentDownloadHigh || currentUploadHigh) {
                // 濡傛灉鐩墠渚濈劧鏄ぇ娴侀噺锛岄噸缃鏃跺櫒
                lowTrafficStartTime = Date.now();
            }
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('娴侀噺鑾峰彇澶辫触:', error);
    }
};

// 閫氳繃鐪熷疄寤惰繜鎺у埗鐘舵€佺伅锛堝姞鍏ュぇ娴侀噺閬胯鍒ゆ柇锛?
const checkNetworkLatency = async () => {
    try {
        const latency = await invoke<number>('get_network_latency');

        // 鍙鑳芥嬁鍒板欢杩熸暟瀛楋紝璇存槑缃戠粶鑲畾鏄€氱殑
        if (latency < 150) {
            networkStatus.value = 'good';      // 寤惰繜浼樼锛岀豢鑹?
        } else {
            networkStatus.value = 'warning';   // 寤惰繜楂?涓嶇ǔ瀹氾紝榛勮壊
        }
    } catch (error) {
        // 褰揜ust鎶涘嚭瓒呮椂寮傚父鏃讹紝璇存槑缃戠粶鍙兘鏂紑杩炴帴

        // 1. 濡傛灉褰撳墠姝ｅ浜庡ぇ娴侀噺鐘舵€侊紝缁濅笉鍙樼孩锛岄檷绾ф樉绀轰负榛勭伅
        if (isHighDownload.value || isHighUpload.value) {
            networkStatus.value = 'warning';
            return;
        }

        // 2. 濡傛灉娴侀噺鍒氬垰娑堝け锛屽垽鏂窛绂诲ぇ娴侀噺缁撴潫鏄惁瓒呰繃浜嗚瀹氱殑缂撳啿鏃堕棿
        const timeSinceLowTraffic = Date.now() - lowTrafficStartTime;
        if (timeSinceLowTraffic < RED_DELAY_MS) {
            // 杩樺湪缂撳啿鏈熷唴锛屽垽瀹氫负澶ф祦閲忓甫鏉ョ殑浣欐尝鍗￠】锛屼緷鐒朵繚鎸侀粍鐏?
            networkStatus.value = 'warning';
        } else {
            // 宸茬粡涓嬩簡濂藉嚑绉掗兘娌℃祦閲忎簡锛岀粨鏋滆繕杩炰笉涓婏紝璇存槑鏄湡鐨勬柇缃戜簡锛屽彉绾紒
            networkStatus.value = 'error';
        }
    }
};

// 鐩戝惉缃戠粶鐘舵€佸彉鍖栵紝瑙﹀彂绯荤粺閫氱煡
watch(networkStatus, (newStatus, oldStatus) => {
    // 蹇界暐鍒濆鍖栨椂鐨勫彉鍖栵紝纭繚鏄湡鐨勭姸鎬佺炕杞?
    if (oldStatus && oldStatus !== newStatus) {
        if (newStatus === 'error') {
            showToast('缃戠粶杩炴帴宸叉柇寮€', 'sys');
        } else if (newStatus === 'good' && oldStatus === 'error') {
            showToast('缃戠粶宸叉仮澶嶈繛鎺?, 'sys');
        }
    }
});

// 璋冩暣绐楀彛浣嶇疆鍒版纭綅缃?
const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 150));
        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = window.devicePixelRatio;

            const WINDOW_INIT_WIDTH = currentWidth.value;   // 榛樿 260
            const WINDOW_INIT_HEIGHT = currentHeight.value; // 榛樿 42
            await appWindow.setSize(new PhysicalSize(Math.ceil(WINDOW_INIT_WIDTH * scaleFactor), Math.ceil(WINDOW_INIT_HEIGHT * scaleFactor)));

            const monitorWidthPhysical = monitor.size.width;
            const monitorLeftPhysical = monitor.position.x;
            const monitorTopPhysical = monitor.position.y;

            // 2. 閲嶆柊鑾峰彇璁惧畾鍚庣殑鐪熷疄鐗╃悊灏哄锛岀敤浜庣簿鍑嗗眳涓?
            const windowSize = await appWindow.innerSize();
            const windowWidthPhysical = windowSize.width;

            const x = monitorLeftPhysical + (monitorWidthPhysical - windowWidthPhysical) / 2;
            const y = monitorTopPhysical + (12 * scaleFactor);

            await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));
        }
    } catch (error) {
        console.error('璋冩暣绐楀彛浣嶇疆澶辫触:', error);
    } finally {
        try {
            await getCurrentWindow().show();
        } catch (e) {
            console.error(e);
        }
    }
};

// 鏍稿績鍔ㄧ敾瀹炵幇锛氬熀浜庝綘鐨?AE 鍏紡杞寲
const onEnter = (el: Element, done: () => void) => {
    const HTMLElement = el as HTMLElement;
    HTMLElement.style.transformOrigin = 'center top'; // 绫讳技鑻规灉鐏靛姩宀涗粠椤堕儴灞曞紑
    let start = performance.now();

    const freq = 2.0;
    const decay = 10.5; // 閫傚害鎷夐珮闃诲姏
    const duration = 600;

    const animate = (time: number) => {
        let t = (time - start) / 1000;
        let progress = (time - start) / duration;

        // 鏁板鏂圭▼锛? - cos(2蟺ft) * e^(-dt)
        let scale = 1 - Math.cos(freq * t * 2 * Math.PI) * Math.exp(-decay * t);
        let opacity = Math.min(1, progress * 4); // 蹇€熸贰鍏?

        HTMLElement.style.transform = `scale(${scale})`;
        HTMLElement.style.opacity = opacity.toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            // 閲嶇疆涓烘渶缁堝共鍑€鐨勭姸鎬?
            HTMLElement.style.transform = `scale(1)`;
            HTMLElement.style.opacity = '1';
            done();
        }
    };
    requestAnimationFrame(animate);
};

const onLeave = (el: Element, done: () => void) => {
    const HTMLElement = el as HTMLElement;
    HTMLElement.style.transformOrigin = 'center top';
    let start = performance.now();

    const duration = 300; // 鏀惰捣鍔ㄧ敾閫氬父鏇村共鑴嗐€佹洿蹇?

    const animate = (time: number) => {
        let progress = (time - start) / duration;

        // 绂诲紑鍔ㄧ敾锛氬揩閫熷钩婊戝洖缂?
        // 浣跨敤 easing 鏇茬嚎鎴栫畝鍖栫殑琛板噺
        let scale = 1 - Math.pow(progress, 3); // 蹇€熷唴鏀?
        let opacity = 1 - progress * 1.5;

        HTMLElement.style.transform = `scale(${Math.max(0, scale)})`;
        HTMLElement.style.opacity = Math.max(0, opacity).toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            done();
            // 绛夊緟 DOM 鍔ㄧ敾鎾斁瀹屾垚鍚庡啀闅愯棌绐楀彛
            getCurrentWindow().hide().catch(console.error);
            // 鍙湁鐢ㄦ埛涓诲姩鍏抽棴鏃舵墠鍚屾鐘舵€佸埌鎺у埗鍙帮紝鑷姩闅愯棌涓嶆敼鍙樺紑鍏崇姸鎬?
            if (!isAutoHiding) {
                emit('island-status-sync', { visible: false });
            }
            isAutoHiding = false;
        }
    };
    requestAnimationFrame(animate);
};

let mouseDownX = 0;
let mouseDownY = 0;
let isMouseDown = false;
let isAutoHiding = false; // 鏍囪褰撳墠闅愯棌鏄惁鐢辫嚜鍔ㄩ殣钘忚Е鍙戯紙鍖哄埆浜庣敤鎴蜂富鍔ㄥ叧闂級

// 鑷畾涔夋í鍚戞嫋鎷界浉鍏崇姸鎬侊紙浠诲姟鏍忔ā寮忎笅浠呭厑璁告í鍚戠Щ鍔級
let isCustomDragging = false;
let customDragStartScreenX = 0;
let customDragStartWindowX = 0;
let customDragStartWindowY = 0;
let customDragMonitor: { position: { x: number; y: number }; size: { width: number; height: number } } | null = null;
let customDragWindowWidth = 0;

const handleMouseDown = (event: MouseEvent) => {
    if ((event.target as HTMLElement).closest('.ctl-btn')) return;
    if ((event.target as HTMLElement).closest('.resize-handle')) return;

    // 妫€娴嬫槸鍚﹀湪杈圭紭鍖哄煙锛屽鏋滄槸鍒欏紑濮嬪搴﹁皟鏁?
    if (!isPositionLocked.value && !isMusicExpanded.value && !isMusicExpanding.value && !isMsgActive.value && !displaySysToast.value) {
        if (isNearEdge(event, 'left')) {
            handleResizeStart(event, 'left');
            return;
        }
        if (isNearEdge(event, 'right')) {
            handleResizeStart(event, 'right');
            return;
        }
    }

    // 鏃犺鏈夋病鏈夐攣瀹氾紝閮藉繀椤昏€佽€佸疄瀹炶褰曞潗鏍囷紝缁欏悗闈㈢殑"鐐瑰嚮灞曞紑"鎻愪緵鍒ゆ柇渚濇嵁锛?
    mouseDownX = event.clientX;
    mouseDownY = event.clientY;
    isMouseDown = true;
};

// ===== 瀹藉害璋冩暣鐩稿叧鍑芥暟 =====
const handleResizeStart = (event: MouseEvent, side: 'left' | 'right') => {
    // 浣嶇疆閿佸畾鏃剁姝㈣皟鏁?
    if (isPositionLocked.value) return;

    // 闊充箰灞曞紑銆佹秷鎭€氱煡绛夌姸鎬佷笅绂佹璋冩暣
    if (isMusicExpanded.value || isMusicExpanding.value || isMsgActive.value || displaySysToast.value) return;

    event.preventDefault();
    event.stopPropagation();

    isResizing.value = true;
    resizeSide.value = side;
    resizeStartX = event.screenX;
    resizeStartWidth = currentWidth.value;

    document.addEventListener('mousemove', handleResizeMove);
    document.addEventListener('mouseup', handleResizeEnd);
};

const handleResizeMove = async (event: MouseEvent) => {
    if (!isResizing.value || !resizeSide.value) return;

    const scaleFactor = window.devicePixelRatio;
    const deltaXLogical = event.screenX - resizeStartX;

    let newWidth: number;
    if (resizeSide.value === 'right') {
        newWidth = resizeStartWidth + deltaXLogical;
    } else {
        newWidth = resizeStartWidth - deltaXLogical;
    }

    // 杈圭晫绾︽潫
    newWidth = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, newWidth));

    // 鏇存柊鐏靛姩宀涘搴?
    try {
        const appWindow = getCurrentWindow();
        await appWindow.setSize(new PhysicalSize(Math.ceil(newWidth * scaleFactor), Math.ceil(currentHeight.value * scaleFactor)));

        // 濡傛灉鏄乏渚ц皟鏁达紝闇€瑕佸悓鏃剁Щ鍔ㄧ獥鍙ｄ綅缃互淇濇寔鍙充晶鍥哄畾
        if (resizeSide.value === 'left') {
            const pos = await appWindow.outerPosition();
            const widthDelta = (newWidth - currentWidth.value) * scaleFactor;
            await appWindow.setPosition(new PhysicalPosition(Math.round(pos.x + widthDelta), Math.round(pos.y)));
        }

        // 鏇存柊褰撳墠瀹藉害
        currentWidth.value = newWidth;
    } catch (error) {
        console.error('璋冩暣瀹藉害澶辫触:', error);
    }
};

const handleResizeEnd = () => {
    isResizing.value = false;
    resizeSide.value = null;
    document.removeEventListener('mousemove', handleResizeMove);
    document.removeEventListener('mouseup', handleResizeEnd);
};

// 淇濆瓨鐢ㄦ埛鑷畾涔夌殑瀹藉害
const saveIslandWidth = () => {
    localStorage.setItem(NSD_ISLAND_WIDTH, String(currentWidth.value));
};

// 鎭㈠鐢ㄦ埛鑷畾涔夌殑瀹藉害
const restoreIslandWidth = () => {
    const saved = localStorage.getItem(NSD_ISLAND_WIDTH);
    if (saved) {
        const width = parseInt(saved, 10);
        if (width >= MIN_WIDTH && width <= MAX_WIDTH) {
            return width;
        }
    }
    return null;
};

// 妫€娴嬮紶鏍囨槸鍚﹀湪鐏靛姩宀涜竟缂橈紙鐢ㄤ簬鏄剧ず璋冩暣鍏夋爣锛?
const isNearEdge = (event: MouseEvent, side: 'left' | 'right'): boolean => {
    if (isPositionLocked.value) return false;

    const target = event.currentTarget as HTMLElement;
    if (!target) return false;

    const rect = target.getBoundingClientRect();
    const EDGE_THRESHOLD = 8; // 杈圭紭妫€娴嬮槇鍊硷紙鍍忕礌锛?

    if (side === 'left') {
        return event.clientX - rect.left <= EDGE_THRESHOLD;
    } else {
        return rect.right - event.clientX <= EDGE_THRESHOLD;
    }
};

const handleMouseMove = async (event: MouseEvent) => {
    // 瀹藉害璋冩暣妯″紡
    if (isResizing.value) {
        await handleResizeMove(event);
        return;
    }

    // 妫€娴嬮紶鏍囨槸鍚﹀湪杈圭紭鍖哄煙锛堢敤浜庡厜鏍囨牱寮忥級
    if (canResize.value) {
        const target = event.currentTarget as HTMLElement;
        if (target) {
            const rect = target.getBoundingClientRect();
            const EDGE_THRESHOLD = 8;
            const leftDist = event.clientX - rect.left;
            const rightDist = rect.right - event.clientX;

            if (leftDist <= EDGE_THRESHOLD && leftDist >= 0) {
                mouseNearEdge.value = 'left';
            } else if (rightDist <= EDGE_THRESHOLD && rightDist >= 0) {
                mouseNearEdge.value = 'right';
            } else {
                mouseNearEdge.value = null;
            }
        }
    } else {
        mouseNearEdge.value = null;
    }

    if (!isMouseDown) return;

    // 1. 鍏ㄥ眬鍔ㄧ敾閿侊細浠讳綍鍙樺舰鍔ㄧ敾鏈熼棿锛岀粷瀵圭姝㈡嫋鎷?
    if (isSizeAnimating) return;

    // 2. 鐘舵€侀攣锛氶煶涔愬睍寮€銆佹秷鎭€氱煡銆佺郴缁熸彁绀烘湡闂达紝缁熺粺绂佹鎷栨嫿锛?
    if (isMusicExpanded.value || isMusicExpanding.value || isMsgActive.value || displaySysToast.value) {
        // 鍙戠幇浼佸浘鎷栨嫿锛岀珛鍒绘墦鏂柦娉?
        isMouseDown = false;
        return;
    }

    // 3. 浣嶇疆宸查攣瀹氭椂锛岀姝竴鍒囨嫋鎷?
    if (isPositionLocked.value) return;

    // 4. 浠诲姟鏍忔ā寮?+ 宸茶В閿侊細浠呭厑璁告í鍚戞嫋鎷斤紙鑷畾涔夊疄鐜帮紝绾︽潫 Y 杞翠笉鍙橈級
    if (isPinnedToTaskbar.value) {
        if (Math.abs(event.clientX - mouseDownX) > 5) {
            isMouseDown = false;
            await startCustomHorizontalDrag(event);
        }
        return;
    }

    // 5. 宀涙ā寮?+ 宸茶В閿侊細鑷敱鎷栨嫿锛堝師鐢?startDragging锛孹/Y 鍧囧彲绉诲姩锛?
    if (Math.abs(event.clientX - mouseDownX) > 5 || Math.abs(event.clientY - mouseDownY) > 5) {
        isMouseDown = false;
        try {
            await getCurrentWindow().startDragging();
        } catch (error) {
            console.error('鎷栨嫿澶辫触:', error);
        }
    }
};

const handleMouseUp = () => {
    // 瀹藉害璋冩暣缁撴潫鏃朵繚瀛樺搴?
    if (isResizing.value) {
        handleResizeEnd();
        saveIslandWidth();
        return;
    }
    isMouseDown = false;
    handleCustomDragEnd();
};

// ===== 鑷畾涔夋í鍚戞嫋鎷斤紙浠诲姟鏍忔ā寮忎笅浠呭厑璁?X 杞寸Щ鍔級=====
const startCustomHorizontalDrag = async (event: MouseEvent) => {
    try {
        const appWindow = getCurrentWindow();
        // 鑾峰彇绐楀彛褰撳墠鐗╃悊鍧愭爣锛屼綔涓烘嫋鎷借捣鐐?
        const pos = await appWindow.outerPosition();
        customDragStartWindowX = pos.x;
        customDragStartWindowY = pos.y;
        customDragStartScreenX = event.screenX;

        // 鑾峰彇鏄剧ず鍣ㄤ俊鎭笌绐楀彛瀹藉害锛岀敤浜庤竟鐣岀害鏉?
        customDragMonitor = await currentMonitor();
        const size = await appWindow.innerSize();
        customDragWindowWidth = size.width;

        isCustomDragging = true;

        // 娣诲姞鏂囨。绾х洃鍚櫒锛岀‘淇濋紶鏍囩Щ鍑虹伒鍔ㄥ矝绐楀彛鍚庝粛鑳芥寔缁拷韪?
        document.addEventListener('mousemove', handleCustomDragMove);
        document.addEventListener('mouseup', handleCustomDragEnd);
    } catch (e) {
        console.error('妯悜鎷栨嫿鍒濆鍖栧け璐?', e);
    }
};

const handleCustomDragMove = async (event: MouseEvent) => {
    if (!isCustomDragging) return;

    const scaleFactor = window.devicePixelRatio;
    const deltaXLogical = event.screenX - customDragStartScreenX;
    const deltaXPhysical = deltaXLogical * scaleFactor;
    let newX = customDragStartWindowX + deltaXPhysical;

    // 杈圭晫绾︽潫锛氶槻姝㈡嫋鍑哄睆骞曞乏鍙宠竟缂?
    if (customDragMonitor) {
        const monitorLeft = customDragMonitor.position.x;
        const monitorRight = customDragMonitor.position.x + customDragMonitor.size.width;
        newX = Math.max(monitorLeft, Math.min(newX, monitorRight - customDragWindowWidth));
    }

    try {
        await getCurrentWindow().setPosition(
            new PhysicalPosition(Math.round(newX), Math.round(customDragStartWindowY))
        );
    } catch (e) {
        console.error('妯悜鎷栨嫿澶辫触:', e);
    }
};

const handleCustomDragEnd = () => {
    if (!isCustomDragging) return;
    isCustomDragging = false;
    document.removeEventListener('mousemove', handleCustomDragMove);
    document.removeEventListener('mouseup', handleCustomDragEnd);
};

// ===== 浣嶇疆鎸佷箙鍖栵紙閿佸畾鏃朵繚瀛橈紝鍚姩鏃舵仮澶嶏級=====
const saveIslandPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        const pos = await appWindow.outerPosition();
        localStorage.setItem(NSD_ISLAND_POSITION, JSON.stringify({ x: pos.x, y: pos.y }));
    } catch (e) {
        console.error('淇濆瓨浣嶇疆澶辫触:', e);
    }
};

const restoreIslandPosition = async (): Promise<boolean> => {
    try {
        const saved = localStorage.getItem(NSD_ISLAND_POSITION);
        if (saved) {
            const { x, y } = JSON.parse(saved);
            await getCurrentWindow().setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));
            return true;
        }
    } catch (e) {
        console.error('鎭㈠浣嶇疆澶辫触:', e);
    }
    return false;
};

const handleRightClick = async (event: MouseEvent) => {
    event.preventDefault();
    event.stopPropagation(); // 闃绘鍐掓场

    // 濡傛灉闊充箰鐏靛姩宀涙鍦ㄥ睍寮€鎴栧凡瀹屽叏灞曞紑锛屽己鍒剁姝㈠懠鍑哄彸閿彍鍗?
    if (isMusicExpanded.value || isMusicExpanding.value || isMsgActive.value || displaySysToast.value) {
        return;
    }

    // 鎵撳紑璁剧疆
    const openSettingsItem = await MenuItem.new({
        text: '鎵撳紑璁剧疆',
        id: 'open_settings',
        action: async () => {
            await emit('open-settings-panel');
            showToast('宸叉墦寮€璁剧疆');
        }
    });

    // 鍒囨崲娴佸厜杈规
    const toggleGlowBorderItem = await MenuItem.new({
        text: isGlowBorderEnabled.value ? '鍏抽棴娴佸厜杈规' : '寮€鍚祦鍏夎竟妗?,
        id: 'toggle_glow_border',
        enabled: true,
        action: () => {
            isGlowBorderEnabled.value = !isGlowBorderEnabled.value;
            localStorage.setItem(NSD_GLOW_BORDER, String(isGlowBorderEnabled.value));
            showToast(isGlowBorderEnabled.value ? '宸插紑鍚祦鍏夎竟妗? : '宸插叧闂祦鍏夎竟妗?);
        }
    });

    // 閲嶇疆浣嶇疆
    const resetPositionItem = await MenuItem.new({
        text: isPinnedToTaskbar.value ? '閲嶇疆浣嶇疆 (宸查攣瀹?' : '閲嶇疆浣嶇疆',
        id: 'reset_position',
        enabled: !isPinnedToTaskbar.value,
        action: async () => {
            try {
                await adjustWindowPosition();
                // 濡傛灉宸查攣瀹氾紝閲嶇疆鍚庨噸鏂颁繚瀛樻柊浣嶇疆
                if (isPositionLocked.value) {
                    await saveIslandPosition();
                }
                showToast('宸查噸缃綅缃?);
            } catch (error) {
                console.error(error);
            }
        }
    });

    // 閲嶇疆瀹藉害
    const resetWidthItem = await MenuItem.new({
        text: '閲嶇疆瀹藉害',
        id: 'reset_width',
        enabled: !isPositionLocked.value,
        action: async () => {
            try {
                // 鍒犻櫎淇濆瓨鐨勮嚜瀹氫箟瀹藉害
                localStorage.removeItem(NSD_ISLAND_WIDTH);
                // 鎭㈠鍒伴粯璁ゅ搴?
                const { w, h } = getBaseSize();
                currentWidth.value = w;
                animateIslandSize(w, h);
                showToast('宸查噸缃搴?);
            } catch (error) {
                console.error(error);
            }
        }
    });

    // 閿佸畾浣嶇疆鑿滃崟椤?
    const toggleLockItem = await MenuItem.new({
        text: isPositionLocked.value ? '瑙ｉ攣 (褰撳墠宸查攣瀹?' : '閿佸畾',
        id: 'toggle_lock',
        enabled: !isPinnedToTaskbar.value,
        action: async () => {
            isPositionLocked.value = !isPositionLocked.value;
            localStorage.setItem(NSD_POSITION_LOCKED, String(isPositionLocked.value));
            // 閿佸畾鏃朵繚瀛樺綋鍓嶄綅缃拰瀹藉害锛屼互渚夸笅娆″惎鍔ㄦ仮澶?
            if (isPositionLocked.value) {
                await saveIslandPosition();
                saveIslandWidth();
            }
            // 鍚屾鐘舵€佺粰璁剧疆闈㈡澘
            await emit('position-lock-sync', { locked: isPositionLocked.value });
            // 鏍规嵁鐘舵€佽Е鍙?lock 鎴?unlock 涓撳睘閫氱煡
            showToast(
                isPositionLocked.value ? '閿佸畾浣嶇疆鎴愬姛' : '浣嶇疆宸茶В閿?,
                isPositionLocked.value ? 'lock' : 'unlock'
            );
        }
    });

    // 鍏抽棴鐏靛姩宀?
    const closeItem = await MenuItem.new({
        text: '鍏抽棴',
        id: 'close',
        action: () => {
            isIslandVisible.value = false;
        }
    });

    // 浣跨敤瀹㈡埛绔潗鏍囪浆閫昏緫鍧愭爣锛堥伩鍏嶆棤杈规瑁佸壀甯︽潵鐨勬紓绉伙級
    const position = new LogicalPosition(
        event.clientX,
        event.clientY
    );

    // 3. 鍒涘缓鑿滃崟骞舵寜椤哄簭杩藉姞杩涘幓
    const menu = await Menu.new();
    await menu.append(openSettingsItem);
    await menu.append(toggleGlowBorderItem);
    await menu.append(resetPositionItem);
    await menu.append(resetWidthItem);
    await menu.append(toggleLockItem);
    await menu.append(closeItem);

    // 4. 寮瑰嚭鑿滃崟
    try {
        isMenuOpen.value = true; // 馃憟 寮瑰嚭鍓嶏紝鍛婅瘔绯荤粺鑿滃崟鎵撳紑浜?
        await menu.popup(position);
    } catch (error) {
        console.error('鑿滃崟寮瑰嚭澶辫触:', error);
    } finally {
        isMenuOpen.value = false; // 馃憟 鏃犺鐢ㄦ埛鏄偣鍑讳簡鑿滃崟锛岃繕鏄偣绌虹櫧澶勫彇娑堜簡锛岄兘浼氱灛闂存仮澶嶇疆椤剁姸鎬?
    }
};

const onInnerEnter = (el: Element, done: () => void) => {
    const htmlEl = el as HTMLElement;
    let start = performance.now();

    // 缁熶竴浣跨敤绠€鍗曠殑娓愬彉娣″叆 (200姣)
    const duration = 180;
    htmlEl.style.transformOrigin = 'center';
    htmlEl.style.opacity = '0';
    htmlEl.style.transform = 'none'; // 纭繚娌℃湁浣嶇Щ

    const animate = (time: number) => {
        let progress = (time - start) / duration;
        htmlEl.style.opacity = Math.min(1, progress).toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            htmlEl.style.opacity = '1';
            done();
        }
    };
    requestAnimationFrame(animate);
};

const onInnerLeave = (el: Element, done: () => void) => {
    const htmlEl = el as HTMLElement;
    let start = performance.now();
    const duration = 140;

    const animate = (time: number) => {
        let progress = (time - start) / duration;
        let opacity = 1 - progress;

        htmlEl.style.opacity = Math.max(0, opacity).toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            done();
        }
    };
    requestAnimationFrame(animate);
};

// 璁板綍鍏ㄥ眬鐏靛姩宀涙槸鍚︽鍦ㄦ墽琛屽舰鍙樺姩鐢?
let isSizeAnimating = false;
let sizeAnimTimer: number | null = null;

// 鐏靛姩宀涙牳蹇冧唬鐮侊紒锛堝畬缇庨槻婕傜Щ+闃茶鍒?闃叉墦鏂姈鍔級
const animateIslandSize = async (targetWidth: number, targetHeight: number) => {
    try {
        // 1. 瑙﹀彂褰㈠彉鍓嶏細绔嬪埢涓婇攣
        isSizeAnimating = true;
        if (sizeAnimTimer) clearTimeout(sizeAnimTimer);

        // 2. 璁惧畾 500ms 鍚庤嚜鍔ㄨВ閿侊紙瑕嗙洊澶у鏁板脊绨у姩鐢荤殑鎸佺画鏃堕棿锛?
        sizeAnimTimer = window.setTimeout(() => {
            isSizeAnimating = false;
        }, 500);

        const appWindow = getCurrentWindow();
        const realSize = await appWindow.innerSize();
        const scaleFactor = window.devicePixelRatio;

        const realStartW = realSize.width / scaleFactor;
        const realStartH = realSize.height / scaleFactor;

        await invoke('start_island_animation', {
            startWidth: realStartW,
            startHeight: realStartH,
            targetWidth: targetWidth,
            targetHeight: targetHeight,
            isPinned: isPinnedToTaskbar.value
        });
    } catch (err) {
        console.error('鍛煎彨 Rust 鍔ㄧ敾澶辫触:', err);
        // 濡傛灉璋冪敤澶辫触锛屽畨鍏ㄨ捣瑙佺珛鍒昏В閿侊紝闃叉姝婚攣
        isSizeAnimating = false;
    }
};

// ===== F11 閫氱煡鐐瑰嚮鎵撳紑锛氬叧闂€氱煡鏄剧ず骞跺惎鍔ㄦ潵婧愬簲鐢?=====
// 鍏抽棴娑堟伅閫氱煡锛屾仮澶嶇伒鍔ㄥ矝鍒伴€氱煡寮瑰嚭鍓嶇殑鐘舵€?
const dismissMsgNotification = () => {
    if (!isMsgActive.value) return;
    if (msgTimer) {
        clearTimeout(msgTimer);
        msgTimer = null;
    }
    isMsgActive.value = false;
    const { h } = getBaseSize();
    const savedWidth = restoreIslandWidth();
    const targetWidth = savedWidth !== null ? savedWidth : currentWidth.value;
    animateIslandSize(targetWidth, h);
    if (isMsgModeEnabled.value) {
        scheduleAutoHide();
    }
};

// 鐐瑰嚮鐏靛姩宀涗笂鐨勯€氱煡锛氱珛鍗宠繑鍥為€氱煡寮瑰嚭鍓嶇姸鎬侊紝骞舵墦寮€鏉ユ簮搴旂敤
const handleNotificationClick = async () => {
    const aumid = msgAumid.value;
    // 鍏堝叧闂€氱煡鏄剧ず锛屾仮澶嶇伒鍔ㄥ矝鐘舵€?
    dismissMsgNotification();
    // 鍐嶅惎鍔ㄦ潵婧愬簲鐢?
    if (aumid) {
        try {
            await invoke('launch_app_by_aumid', { aumid });
        } catch (e) {
            console.error('鎵撳紑鏉ユ簮搴旂敤澶辫触:', e);
        }
    }
};

// 鍔ㄧ敾閿佷笌绛夊緟闃熷垪鏍囧織
let isAnimationLocked = false;
let isPendingCollapse = false;

// 闊充箰鎺у埗鍣ㄨ嚜鍔ㄦ敹缂╂柟娉?
const collapseMusic = () => {
    if (!isMusicExpanded.value && !isMusicExpanding.value) return;

    // 銆愭牳蹇冮€昏緫銆戯細濡傛灉姝ｅ湪鐚涚儓灞曞紑涓紝缁濆涓嶆墦鏂紒鎶婃敹缂╄姹傛寕璧凤紝绛夊畠灞曞紑瀹岃嚜鍔ㄦ墽琛屻€?
    if (isAnimationLocked) {
        isPendingCollapse = true;
        return;
    }

    isMusicExpanded.value = false;
    isMusicExpanding.value = false;
    isPendingCollapse = false; // 娓呴櫎闃熷垪

    if (musicExpandAnimTimer) {
        clearTimeout(musicExpandAnimTimer);
        musicExpandAnimTimer = null;
    }

    // 鎶樺彔鏃舵仮澶嶇敤鎴疯嚜瀹氫箟鐨勫搴︼紝鑰屼笉鏄娇鐢ㄩ粯璁ゅ搴?
    const { h } = getBaseSize();
    const savedWidth = restoreIslandWidth();
    const targetWidth = savedWidth !== null ? savedWidth : currentWidth.value;
    animateIslandSize(targetWidth, h);
};

// 闊充箰鎺у埗鍣ㄧ偣鍑诲睍寮€鏂规硶
const expandMusic = (e: MouseEvent) => {
    if (Math.abs(e.clientX - mouseDownX) > 5 || Math.abs(e.clientY - mouseDownY) > 5) return;
    if ((e.target as HTMLElement).closest('.ctl-btn')) return;

    if (isMusicExpanded.value || isMusicExpanding.value) return;

    isMusicExpanding.value = true;
    isPendingCollapse = false;  // 閲嶇疆寰呭姙浠诲姟
    isAnimationLocked = true;   // 鈿?涓婇攣锛佸甯冭繘鍏ョ鍦ｄ笉鍙镜鐘殑灞曞紑鍛ㄦ湡

    // 1. 寮规€ф寜鍘嬪姩鐢?(鍏堝井寰彉灏?
    animateIslandSize(245, 38);

    // 2. 寤惰繜 120 姣鍚庯紝鎵撴柇缂╁皬锛岀洿鎺ョ寷鐑堝睍寮€
    musicExpandAnimTimer = window.setTimeout(() => {
        isMusicExpanded.value = true;
        isMusicExpanding.value = false;
        animateIslandSize(320, 150);

        // 3. 鏍规嵁 Rust 绔殑寮圭哀琛板噺棰戠巼锛岀害 400ms 鍚庡姩鐢诲交搴曠粨鏉燂紝姝ゆ椂瑙ｉ攣
        setTimeout(() => {
            isAnimationLocked = false;

            // 妫€鏌ワ細濡傛灉鍦ㄥ睍寮€鐨勮繖 520ms 閲岋紝鐢ㄦ埛榧犳爣宸茬粡绉昏蛋浜嗭紝閭ｅ氨绔嬪埢琛ュ彂鏀剁缉鍛戒护锛?
            if (isPendingCollapse) {
                isPendingCollapse = false;
                collapseMusic();
            }
        }, 400);
    }, 120);
};

// 榧犳爣绂诲紑鐏靛姩宀涙椂锛氳嚜鍔ㄦ姌鍙犳垨鑷姩闅愯棌
const handleMouseLeave = () => {
    // 娓呴櫎榧犳爣杈圭紭妫€娴嬬姸鎬?
    mouseNearEdge.value = null;
    isMouseOver.value = false;

    // 1. 鑷姩鎶樺彔閫昏緫锛氬綋鐏靛姩宀涘睍寮€鏃讹紝榧犳爣绂诲紑鍚庡欢杩熸姌鍙犲洖灏忓矝鐘舵€?
    if (isAutoCollapseEnabled.value && (isMusicExpanded.value || isMusicExpanding.value)) {
        // 鍚姩鑷姩鎶樺彔瀹氭椂鍣?
        if (autoCollapseTimer) {
            clearTimeout(autoCollapseTimer);
            autoCollapseTimer = null;
        }
        autoCollapseTimer = window.setTimeout(() => {
            if (!isMouseOver.value && (isMusicExpanded.value || isMusicExpanding.value)) {
                collapseMusic();
            }
        }, autoCollapseDelay.value);
    }

    // 2. 鑷姩闅愯棌閫昏緫锛氬綋娌℃湁娲诲姩鏃讹紝寤惰繜闅愯棌鐏靛姩宀?
    if (!isAutoHideEnabled.value) {
        return;
    }

    // 妫€鏌ユ槸鍚︽湁娲诲姩鐘舵€侊紙娑堟伅銆侀煶涔愬睍寮€銆佺郴缁熼€氱煡锛?
    const hasActivity = isMsgActive.value || isMusicExpanded.value || isMusicExpanding.value || displaySysToast.value;
    
    // 濡傛灉鏈夋椿鍔紝涓嶈Е鍙戣嚜鍔ㄩ殣钘?
    if (hasActivity) {
        return;
    }

    // 鍚姩鑷姩闅愯棌瀹氭椂鍣?
    scheduleAutoHide();
};

// 榧犳爣閲嶆柊绉诲叆鐏靛姩宀涙椂锛氱珛鍒绘墦鏂敹缂╀紒鍥?
const handleMouseEnter = () => {
    // 濡傛灉涔嬪墠绉诲嚭鐣欎笅浜嗘敹缂╂搴曪紝浣嗗姩鐢昏繕娌℃挱瀹岄紶鏍囧張鍥炴潵浜嗭紝鐩存帴鍙栨秷杩欎釜妗堝簳
    isPendingCollapse = false;
    isMouseOver.value = true;

    // 鍙栨秷鑷姩闅愯棌瀹氭椂鍣?
    if (autoHideTimer) {
        clearTimeout(autoHideTimer);
        autoHideTimer = null;
    }

    // 鍙栨秷鑷姩鎶樺彔瀹氭椂鍣?
    if (autoCollapseTimer) {
        clearTimeout(autoCollapseTimer);
        autoCollapseTimer = null;
    }
};

watch(displayMusic, (newVal: boolean) => {
    if (!newVal) {
        collapseMusic(); // 涓€鏃﹂煶涔愬矝琚殣钘忥紙涓嶇鏄洜涓鸿疆鎹㈣繕鏄墜鍔ㄥ叧浜嗭級锛岀珛鍒绘敹缂?
    }
});

// 寮曞叆浣犵殑榛樿鍥炬爣浣滀负鍏滃簳
import defaultLogo from '../assets/logo.png';
const currentMsgIcon = ref(defaultLogo);

// 鍥炬爣鏄犲皠鍣?
const getAppIcon = (appName: string) => {
    const name = appName.toLowerCase();

    if (name.includes('qq')) {
        // 浣跨敤 new URL 璁?Vite 鐭ラ亾浣犺寮曞叆杩欎釜璧勬簮
        return new URL('../assets/qq.png', import.meta.url).href;
    }
    if (name.includes('閽夐拤') || name.includes('dingtalk')) {
        return new URL('../assets/dingtalk.png', import.meta.url).href;
    }
    if (name.includes('mail') || name.includes('閭欢')) {
        return new URL('../assets/mail.png', import.meta.url).href;
    }
    if (name.includes('wechat') || name.includes('寰俊')) {
        return new URL('../assets/wechat.png', import.meta.url).href;
    }

    return defaultLogo;
};

onMounted(async () => {
    // widget 鍙兘鍦ㄤ富闈㈡澘鏈垱寤烘垨鐪佸唴瀛橀攢姣佸悗鐙珛杩愯锛岄渶鑷鎭㈠鐩爣鎾斁鍣ㄣ€?
    await invoke('set_target_player', {
        player: localStorage.getItem(NSD_TARGET_PLAYER) || 'netease',
    }).catch(() => {});

    window.addEventListener('blur', collapseMusic);

    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true }); // 浣跨敤鎹曡幏闃舵锛岀‘淇濆厛浜?Tauri 搴曞眰鎷︽埅

    // 闊充箰鎺у埗鍣ㄧ姸鎬佺洃鍚櫒
    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        const isEnabled = event.payload.enabled;
        isMusicCtlEnabled.value = isEnabled;

        if (isEnabled) {
            // 鍒ゆ柇鏄笉鏄?棣栨"锛堟湰鍦版湁娌℃湁瀛樿繃娴佸厜杈规鐨勬暟鎹級
            if (localStorage.getItem(NSD_GLOW_BORDER) === null) {
                isGlowBorderEnabled.value = true; // 鑷姩寮€鍚祦鍏夎竟妗?
                localStorage.setItem(NSD_GLOW_BORDER, 'true'); // 瀛樺叆璁板繂锛屼互鍚庡氨涓嶇畻"棣栨"浜?
            }

            showInfo.value = false;
            musicBoxKey.value++;
            
            // 闊充箰鎾斁鍣ㄦā寮忥細寮€鍚椂濡傛灉娌℃湁闊充箰鎾斁锛岄殣钘忕伒鍔ㄥ矝锛堢被浼奸€氱煡妯″紡锛?
            if (!isPlaying.value && isIslandVisible.value && !isMouseOver.value) {
                scheduleAutoHide();
            }
        }
    });

    // 鐩戝惉绯荤粺搴曞眰浜嬩欢锛堥煶閲忋€佺數婧愶級
    await listen<string>('system-event', (event) => {
        showToast(event.payload, 'sys');
    });

    await listen<{ state: 'charging' | 'discharging', percent: number }>('battery-event', (event) => {
        const { state, percent } = event.payload;

        if (state === 'charging') {
            showToast(`宸叉帴鍏ョ數婧愶紝褰撳墠鐢甸噺 ${percent}%`, 'battery-charge');
        } else if (state === 'discharging' && percent <= 20) {
            // 杩欓噷杩樺彲浠ュ姞鍏ラ槻鎶栵細鍙湪鍒氭帀鍒?20%銆?0%銆?% 绛夊叧閿妭鐐硅Е鍙戜竴娆★紝閬垮厤鐤媯寮圭獥
            showToast(`鐢垫睜鐢甸噺浣庯紝鍓╀綑 ${percent}%`, 'battery-low');
        }
    });

    // 鐩戝惉鏉ヨ嚜鎺у埗鍙扮殑閫忔槑搴﹀悓姝ユ寚浠?
    await listen<{ opacity: number }>('control-island-opacity', (event) => {
        islandOpacity.value = event.payload.opacity;
    });

    // 鐩戝惉鏉ヨ嚜鎺у埗鍙扮殑涓婚鍚屾鎸囦护
    await listen<{ theme: string }>('control-island-theme', (event) => {
        islandTheme.value = event.payload.theme;
    });

    // 鐩戝惉缃簬浠诲姟鏍忓紑鍏?
    await listen<{ enabled: boolean }>('control-pin-taskbar', async (event) => {
        isPinnedToTaskbar.value = event.payload.enabled;
        if (isPinnedToTaskbar.value) {
            await snapToBottomLeft(); // 寮€鍚椂锛氶鍒板乏涓嬭
        } else {
            await adjustWindowPosition(); // 鍏抽棴鏃讹細绛夊悓浜庣偣鍑?閲嶇疆浣嶇疆"锛岄鍥為《閮ㄥ眳涓?
        }
        // 濡傛灉浣嶇疆宸查攣瀹氾紝妯″紡鍒囨崲鍚庨噸鏂颁繚瀛樻柊浣嶇疆锛堥伩鍏嶄笅娆″惎鍔ㄦ仮澶嶅埌杩囨湡鍧愭爣锛?
        if (isPositionLocked.value) {
            await saveIslandPosition();
        }
    });

    // 鐩戝惉鏉ヨ嚜璁剧疆闈㈡澘鐨勪綅缃攣瀹氫俊鍙?
    await listen<{ locked: boolean }>('control-position-lock', async (event) => {
        isPositionLocked.value = event.payload.locked;
        // 閿佸畾鏃朵繚瀛樺綋鍓嶄綅缃紝浠ヤ究涓嬫鍚姩鎭㈠
        if (isPositionLocked.value) {
            await saveIslandPosition();
        }
    });

    // 鐩戝惉娑堟伅妯″紡寮€鍏?
    await listen<{ enabled: boolean }>('control-msg-mode', async (event) => {
        isMsgModeEnabled.value = event.payload.enabled;
        if (isMsgModeEnabled.value && !isMsgActive.value) {
            // 濡傛灉寮€鍚簡娑堟伅妯″紡锛屽苟涓斿綋鍓嶆病鏈夋秷鎭紝寤惰繜闅愯棌
            if (autoHideTimer) {
                clearTimeout(autoHideTimer);
                autoHideTimer = null;
            }
            autoHideTimer = window.setTimeout(() => {
                if (!isMouseOver.value && isIslandVisible.value) {
                    isAutoHiding = true;
                    isIslandVisible.value = false;
                }
            }, autoHideDelay.value);
        } else if (!isMsgModeEnabled.value) {
            // 濡傛灉鍏抽棴浜嗘秷鎭ā寮忥紝绔嬪埢鎭㈠鏄剧ず
            await getCurrentWindow().show();
            isIslandVisible.value = true;

            // 閫氱煡鎺у埗鍙版仮澶嶅紑鍏崇姸鎬侊紝璁╀富闈㈡澘鐨勫紑鍏冲悓姝ュ彉缁匡紙寮€鍚級
            await emit('island-status-sync', { visible: true });
        }
    });

    // 鐩戝惉杞崲妯″紡寮€鍏?
    await listen<{ enabled: boolean }>('control-rotation-mode', (event) => {
        isRotationEnabled.value = event.payload.enabled;
        if (isRotationEnabled.value) {
            startRotation();
        } else {
            stopRotation();
            currentRotIndex.value = 0; // 鍏抽棴鏃堕噸缃洖缃戦€?
        }
    });

    // 鐩戝惉鑷姩闅愯棌璁剧疆
    await listen<{ enabled: boolean, delay: number }>('control-auto-hide', (event) => {
        isAutoHideEnabled.value = event.payload.enabled;
        autoHideDelay.value = event.payload.delay;
        localStorage.setItem(NSD_AUTO_HIDE_ENABLED, String(isAutoHideEnabled.value));
        localStorage.setItem(NSD_AUTO_HIDE_DELAY, String(autoHideDelay.value));
    });

    // 鐩戝惉鑷姩鎶樺彔璁剧疆
    await listen<{ enabled: boolean, delay: number }>('control-auto-collapse', (event) => {
        isAutoCollapseEnabled.value = event.payload.enabled;
        autoCollapseDelay.value = event.payload.delay;
        localStorage.setItem(NSD_AUTO_COLLAPSE_ENABLED, String(isAutoCollapseEnabled.value));
        localStorage.setItem(NSD_AUTO_COLLAPSE_DELAY, String(autoCollapseDelay.value));
    });

    // 鍚姩鏃跺鏋滃紑浜嗚疆鎹紝灏辫窇璧锋潵
    if (isRotationEnabled.value) {
        startRotation();
    }

    // 鍒濆鍖栦綅缃拷韪?
    const appWindow = getCurrentWindow();
    try {
        await appWindow.innerPosition();
    } catch (e) { }

    // 鍦ㄥ惎鍔ㄨ皟鏁翠綅缃墠锛屾牴鎹綋鍓嶇殑瀹為檯鐘舵€侊紝鏍″噯鍒濆瀹介珮
    const { w, h } = getBaseSize();
    // 浼樺厛鎭㈠鐢ㄦ埛鑷畾涔夌殑瀹藉害
    const savedWidth = restoreIslandWidth();
    currentWidth.value = savedWidth !== null ? savedWidth : w;
    currentHeight.value = h;

    // 绔嬪嵆璁剧疆绐楀彛澶у皬锛岀‘淇濆搴︽仮澶嶇敓鏁?
    try {
        const appWindow = getCurrentWindow();
        const scaleFactor = window.devicePixelRatio;
        await appWindow.setSize(new PhysicalSize(Math.ceil(currentWidth.value * scaleFactor), Math.ceil(currentHeight.value * scaleFactor)));
    } catch (error) {
        console.error('璁剧疆鍒濆绐楀彛澶у皬澶辫触:', error);
    }

    // 鏍规嵁鏈湴璁板綍鍐冲畾鍚姩鏃跺嚭鐜板湪鍝?
    if (isPositionLocked.value) {
        // 宸查攣瀹氫綅缃細灏濊瘯鎭㈠涓婃淇濆瓨鐨勫潗鏍?
        const restored = await restoreIslandPosition();
        if (!restored) {
            // 娌℃湁淇濆瓨杩囦綅缃紝鍥為€€鍒伴粯璁ゅ畾浣?
            if (isPinnedToTaskbar.value) {
                await snapToBottomLeft();
            } else {
                await adjustWindowPosition();
            }
        }
    } else {
        // 鏈攣瀹氾細浣跨敤榛樿瀹氫綅
        if (isPinnedToTaskbar.value) {
            await snapToBottomLeft();
        } else {
            await adjustWindowPosition();
        }
    }

    // 鍏堟樉绀洪€忔槑鐨?Tauri 绐楀彛锛屽啀瑙﹀彂 Vue 鐨勭伒鍔ㄥ矝鍏ュ満寮圭哀鍔ㄧ敾
    // 濡傛灉娌″紑娑堟伅妯″紡锛屾墠鍦ㄥ惎鍔ㄦ椂鐩存帴鏄剧ず鐏靛姩宀?
    if (!isMsgModeEnabled.value) {
        await getCurrentWindow().show();
        isIslandVisible.value = true;
    }

    // 鐩戝惉鏉ヨ嚜鎺у埗鍙扮殑绯荤粺纭欢鐩戞帶寮€鍏?
    await listen<{ enabled: boolean }>('control-hardware-mon', (event) => {
        isHardwareMonEnabled.value = event.payload.enabled;
    });

    fetchSpeedStats();
    checkNetworkLatency();

    // 鍚姩缃戦€熷拰纭欢鏄剧ず杞崲瀹氭椂鍣?(姣?5 绉掑垏鎹竴娆?
    speedCycleTimer = window.setInterval(() => {
        // 缃戦€熻疆鎹?
        if (displaySpeed.value) {
            isShowingUpload.value = !isShowingUpload.value;
        }
        // 纭欢杞崲
        if (displayHardware.value) {
            isShowingCPU.value = !isShowingCPU.value;
        }
    }, 5000);

    // 鍦ㄤ綘鍘熸湁鐨勬瘡绉掑埛鏂板畾鏃跺櫒涓紝椤哄甫鎵ц闊充箰鍚屾
    // 1. 楂橀瀹氭椂鍣細涓撻棬璐熻矗缃戦€熷拰纭欢鐩戞帶锛堟瘡 500ms ~ 1000ms 鍒锋柊涓€娆★級
    speedTimer = setInterval(async () => {
        // 寮虹疆椤堕€昏緫锛堟嫋鎷戒腑璺宠繃锛岄伩鍏嶄笌 setPosition 鍐茬獊锛?
        if (isPinnedToTaskbar.value && isIslandVisible.value && !isMenuOpen.value && !isCustomDragging) {
            invoke('force_window_topmost').catch(() => { });
        }

        // 鍒锋柊缃戦€?
        fetchSpeedStats();

        // 鍒锋柊纭欢鐘舵€?
        if (isHardwareMonEnabled.value || isRotationEnabled.value) {
            try {
                const [cpu, usedMem, totalMem] = await invoke<[number, number, number]>('get_hardware_stats');
                cpuUsage.value = Math.round(cpu) + '%';
                if (totalMem > 0) {
                    memUsage.value = Math.round((usedMem / totalMem) * 100) + '%';
                }
            } catch (err) {
                console.error('鑾峰彇纭欢淇℃伅澶辫触:', err);
            }
        }
    }, 800) as unknown as number;


    // 2. 涓瀹氭椂鍣細涓撻棬璐熻矗闊充箰鐘舵€佸悓姝ワ紙姣?2000ms 鍒锋柊涓€娆″嵆鍙級
    musicTimer = setInterval(() => {
        if (isMusicCtlEnabled.value || isRotationEnabled.value) {
            syncMusicStatus();
        }
    }, 2000);


    // 3. 浣庨瀹氭椂鍣細涓撻棬杞绯荤粺閫氱煡锛堥€氱煡涓嶉渶瑕佹姠鏃堕棿锛?.5绉掓崲鏉ユ瀬浣庣殑璧勬簮鍗犵敤锛?
    notifyTimer = setInterval(async () => {
        const enabled = localStorage.getItem(NSD_MSG_NOTIFY) === 'true';
        if (!enabled) return;

        try {
            const res = await invoke<any>('fetch_latest_notification');
            if (res) {
                msgAumid.value = res.aumid;

                // 鏍囬鍙瓨鍙戦€佽€咃紙濡傛灉娌℃湁鍗曠嫭鏍囬灏辨樉绀?'鏂伴€氱煡'锛?
                msgTitle.value = (res.title && res.title !== res.app_name) ? res.title : '鏂伴€氱煡';
                // 鍗曠嫭鎶婄▼搴忓悕瀛樿捣鏉?
                msgAppName.value = res.app_name;
                // 鍐呭鍏滃簳閫昏緫淇濇寔涓嶅彉
                msgBody.value = res.body || (res.title === res.app_name ? '鏀跺埌涓€鏉℃柊閫氱煡' : res.title);

                currentMsgIcon.value = getAppIcon(res.app_name);

                if (!isMsgActive.value) {
                    isMsgActive.value = true;
                    // 鑷姩鎭㈠鏄剧ず锛氬綋鏈夋秷鎭椿鍔ㄦ椂锛屽鏋滅伒鍔ㄥ矝琚殣钘忥紝鍒欒嚜鍔ㄦ仮澶嶆樉绀?
                    if (!isIslandVisible.value) {
                        getCurrentWindow().show();
                        isIslandVisible.value = true;
                    }
                    if (isMsgModeEnabled.value && !isIslandVisible.value) {
                        getCurrentWindow().show();
                        isIslandVisible.value = true;
                    }
                    if (!isPinnedToTaskbar.value) {
                        animateIslandSize(360, 65);
                    }
                }

                if (msgTimer) clearTimeout(msgTimer);
                msgTimer = setTimeout(() => {
                    isMsgActive.value = false;
                    const { h } = getBaseSize();
                    const savedWidth = restoreIslandWidth();
                    const targetWidth = savedWidth !== null ? savedWidth : currentWidth.value;
                    animateIslandSize(targetWidth, h);
                    if (isMsgModeEnabled.value) {
                        scheduleAutoHide();
                    }
                }, 5000);
            }
        } catch (err) {
            console.error(err);
        }
    }, 2500);

    // 璋冨ぇPing闂撮殧锛氫粠2.5绉掕皟澶у埌5.5绉?
    pingTimer = setInterval(checkNetworkLatency, 5500) as unknown as number;

    // 鐩戝惉鎺у埗鍙板彂鏉ョ殑鏄鹃殣璋冨害鎸囦护
    await listen<{ show: boolean }>('control-island-visibility', async (event) => {
        if (event.payload.show) {
            // 1. 鍏堣閫忔槑鐨?OS 绐楀彛瀹瑰櫒鏄剧ず锛屾鏃跺唴閮?DOM 涓?v-show="false"锛岃瑙変笂浠嶆槸闅愬舰鐨?
            await getCurrentWindow().show();
            await getCurrentWindow().setAlwaysOnTop(true);
            // 2. 缁欎簣 40ms 鐨勬祻瑙堝櫒娓叉煋甯х紦鍐诧紝鍐嶆挄寮€ Vue 鐨?v-show 鐘舵€侊紝寮哄埗瑙﹀彂 enter 鍔ㄧ敾
            setTimeout(() => {
                isIslandVisible.value = true;
            }, 40);
        } else {
            // 鎺у埗鍙板叧闂寚浠?-> 瑙﹀彂甯歌绂诲紑鍔ㄧ敾
            isIslandVisible.value = false;
        }
    });

    // 瀹炴椂鐩戝惉鏉ヨ嚜 Rust 搴曞眰鍙戞潵鐨勬竻閫忓儚绱犳祦锛屾棤缂濆悓姝ョ粰 Vue 鐨勫搷搴斿紡 DOM 瀹介珮
    await listen<number[]>("island-resize", (event) => {
        const [w, h] = event.payload;
        currentWidth.value = w;
        currentHeight.value = h;
    });

    // B8: 鐩戝惉鍚庣鎺ㄦ潵鐨勯璋辨暟鎹紙鏇夸唬 50ms setInterval 杞锛屾樉钁楀噺灏?IPC 璋冪敤娆℃暟锛?
    await listen<number[]>("spectrum-data", (event) => {
        spectrumData.value = event.payload;
    });

    // 鍒濆鍖栨椂瑙﹀彂涓€娆¤绠?
    setTimeout(() => {
        calculateScroll();
    }, 700);
});

onUnmounted(() => {
    window.removeEventListener('blur', collapseMusic);
    // 娓呯悊鑷畾涔夋í鍚戞嫋鎷界殑鏂囨。绾х洃鍚櫒
    document.removeEventListener('mousemove', handleCustomDragMove);
    document.removeEventListener('mouseup', handleCustomDragEnd);
    clearInterval(speedTimer);
    clearInterval(pingTimer);
    stopRotation();
    clearInterval(musicTimer);
    clearInterval(notifyTimer);
    stopProgressTimer();
    // 缁勪欢鍗歌浇鏃跺叧闂璋辨崟鑾凤紝閬垮厤鍚庣绌鸿窇
    invoke('set_spectrum_active', { active: false }).catch(() => {});
    if (speedCycleTimer) clearInterval(speedCycleTimer);
});
</script>

<style scoped>
*,
*::before,
*::after {
    box-sizing: border-box;
    border: none !important;
    outline: none !important;
}

:root {
    -webkit-app-region: drag;
}

:global(html),
:global(body) {
    background-color: transparent !important;
    background: transparent !important;
    overflow: hidden;
    margin: 0;
    padding: 0;
    border: none !important;
}

/* 澶栧眰鍖呰９灞傦細璐熻矗瑁佸垏澶氫綑鐨勬祦鍏?*/
.island-container {
    /* 绉婚櫎 position: absolute; top: 0; */
    margin: 0 auto;
    /* 璁╁畠鍦ㄧ獥鍙ｅ唴姘村钩灞呬腑 */
    border-radius: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    user-select: none;
    -webkit-user-select: none;
    overflow: hidden;
    background: transparent;
    transition: background 0.4s ease;
    box-sizing: border-box;
    transform: translateZ(0);
    will-change: width, height, border-radius;
    contain: strict;
}

/* 闅愯棌鍦ㄥ簳灞傜殑宸ㄥぇ鏃嬭浆娓愬彉灞?*/
.rainbow-border-glow {
    position: absolute;
    width: 500px;
    height: 500px;

    /* 淇鏃嬭浆涓績鍋忕Щ闂 */
    top: calc(50% - 250px);
    left: calc(50% - 250px);

    z-index: 1;

    /* 閲嶆柊缁樺埗鐨勫畬缇庡绉扮幆褰㈡笎鍙橈紝娓呴€忎笉鍙戣剰 */
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='500' height='500'%3E%3Cdefs%3E%3Cfilter id='b' x='-50%25' y='-50%25' width='200%25' height='200%25'%3E%3CfeGaussianBlur in='SourceGraphic' stdDeviation='60'/%3E%3C/filter%3E%3C/defs%3E%3Cg filter='url(%23b)'%3E%3Ccircle cx='250' cy='90' r='150' fill='%23ff3b30'/%3E%3Ccircle cx='390' cy='170' r='150' fill='%23ff9500'/%3E%3Ccircle cx='390' cy='330' r='150' fill='%234cd964'/%3E%3Ccircle cx='250' cy='410' r='150' fill='%23007aff'/%3E%3Ccircle cx='110' cy='330' r='150' fill='%235856d6'/%3E%3Ccircle cx='110' cy='170' r='150' fill='%23ff2d55'/%3E%3C/g%3E%3C/svg%3E");
    background-size: cover;

    /* 10绉掍竴鍦堝垰鍒氬ソ锛屾煍鍜屼笖涓嶆€庝箞鍚?GPU */
    animation: rainbow-rotate 10s linear infinite;
    will-change: transform;
}

/* 鏍稿績閬僵鍐呭鍧楋細鎸″湪鏃嬭浆娓愬彉灞傜殑涓婃柟 */
.island-core-content {
    position: relative;
    z-index: 2;
    width: 100%;
    height: 100%;
    border-radius: 98px;
    transform: translateZ(0);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    overflow: hidden;
}

/* 椤烘椂閽堝寑閫熸棆杞?*/
@keyframes rainbow-rotate {
    from {
        transform: rotate(0deg);
    }

    to {
        transform: rotate(360deg);
    }
}

[data-tauri-drag-region] {
    -webkit-app-region: drag;
    cursor: grab;
}

[data-tauri-drag-region]:active {
    cursor: grabbing;
}

/* 淇敼缃戦€熺洅瀛愬竷灞€锛屽己鍒堕潬宸︼紝骞跺姞鍏ュ乏渚у唴杈硅窛 */
.speed-box {
    position: absolute;
    left: 0;
    top: 0;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    width: 100%;
    height: 100%;
}

.speed-item {
    display: flex;
    align-items: center;
    gap: 6px;
    /* 绋嶅井鎷夊紑绠ご鍜屾暟瀛楃殑璺濈 */
    transform: translateY(-1px);
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
}

.label {
    font-size: 10px;
    /* 绋嶅井璋冨ぇ绠ご */
    color: currentColor;
    opacity: 0.5;
    font-weight: 800;
    padding: 2px 5px;
    border-radius: 4px;
    transition: all 0.3s ease;
    background: rgba(150, 150, 150, 0.15);
    /* 榛樿缁欎竴涓贰娣＄殑搴曡壊锛屽鍔犺川鎰?*/
}

/* 楂樻祦閲忔椂鐨?label 鏍峰紡 */
.label.high-traffic {
    color: currentColor;
    opacity: 1;
    background: rgba(255, 255, 255, 0.25);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .label.high-traffic {
    background: rgba(0, 0, 0, 0.15);
}

.value {
    font-size: 12px;
    transform: translateY(-0.5px);
    font-weight: 600;
    letter-spacing: 0.2px;
    font-variant-numeric: tabular-nums;
    min-width: 65px;
    text-align: left;
}

/* 缃戦€熻疆鎹㈢殑娣″叆娣″嚭鍔ㄧ敾 */
.speed-fade-enter-active,
.speed-fade-leave-active {
    transition: opacity 0.3s ease, transform 0.3s ease;
}

.speed-fade-enter-from {
    opacity: 0;
    transform: translateY(4px);
    /* 寰井浠庝笅鏂规粦鍏?*/
}

.speed-fade-leave-to {
    opacity: 0;
    transform: translateY(-4px);
    /* 寰井鍚戜笂婊戝嚭 */
}

.status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    transition: background-color 0.4s ease;
}

/* 鍘绘帀鍙戝厜闃村奖锛屾敼涓虹函绮圭殑鎵佸钩鍖栧渾鐐癸紝骞插噣鍒╄惤 */
.good {
    background-color: #34C759;
}

.warning {
    background-color: #FFCC00;
}

.error {
    background-color: #FF3B30;
}

/* 璁╀袱涓洅瀛愯劚绂诲郊姝ょ殑褰卞搷锛屽湪鍚屼竴涓寘瑁瑰眰鍐呭畬缇庣殑鈥滈噸鍙犫€濇斁缃?*/
.music-ctl-box,
.speed-box {
    position: absolute;
    /* 鏀逛负缁濆瀹氫綅锛屽疄鐜版棤缂濆钩鏇?*/
    left: 0;
    top: 0;
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
}

.music-ctl-box {
    justify-content: flex-start;
}

/* 澧炲姞缁熶竴鐨勫唴閮ㄧ粷瀵瑰畾浣嶅钩鏇垮寘瑁瑰眰 */
.inner-wrapper {
    position: relative;
    flex-grow: 1;
    height: 100%;
    display: flex;
    align-items: center;
}

.album-cover {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    box-sizing: unset !important;
    border: 2px solid rgba(255, 255, 255, 0.5) !important;
    background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%);
    flex-shrink: 0;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 0 10px rgba(255, 255, 255, 0.250);
    transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    z-index: 2;
    transform: translateX(-8px);
}

/* 浜壊妯″紡涓嬬殑澶栫幆棰滆壊鑷姩鍙樻殫 */
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .album-cover {
    border-color: rgba(0, 0, 0, 0.15);
}

.album-cover.is-playing {
    transform: scale(1.08) translateX(-8px);
}

/* 灏侀潰鍐呴儴缁戝畾鑳屾櫙鍥剧殑 div */
.cover-inner {
    width: 100%;
    height: 100%;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    transition: background-image 0.3s ease;
    animation: rotate 8s linear infinite;
    animation-play-state: paused;
    /* 榛樿璁╁姩鐢诲浜庢殏鍋滅姸鎬?*/
}

/* 姝ｅ湪鎾斁鏃剁殑鏃嬭浆鍔ㄧ敾 */
.is-playing .cover-inner {
    animation-play-state: running;
    /* 褰撴湁鎾斁鐘舵€佹椂锛岃鍔ㄧ敾璺戣捣鏉?*/
}

@keyframes rotate {
    from {
        transform: rotate(0deg);
    }

    to {
        transform: rotate(360deg);
    }
}

.music-controls {
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    align-items: center;
    gap: 12px;
    z-index: 10;
}

.ctl-btn {
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 6px;
    border-radius: 50%;
    transition: background-color 0.2s ease, opacity 0.2s ease, transform 0.1s ease;
    outline: none;
    -webkit-app-region: no-drag;
}

/* 鍙湁鍦?hover 鐨勬椂鍊欐墠鍑虹幇鑳屾櫙鑹?*/
.ctl-btn:hover {
    background-color: rgba(255, 255, 255, 0.15);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .ctl-btn:hover {
    background-color: rgba(0, 0, 0, 0.1);
}

.ctl-btn:active {
    opacity: 0.6;
    transform: scale(0.92);
}

.ctl-btn svg {
    width: 16px;
    height: 16px;
    pointer-events: none;
}

/* 鎾斁閿◢寰瘮鍒囨瓕閿ぇ涓€鐐圭偣锛岀獊鍑鸿瑙変腑蹇?*/
.play-btn svg {
    width: 20px;
    height: 20px;
}

/* 鎺т欢鏄鹃殣娣″叆娣″嚭鍔ㄧ敾杩囨浮 */
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

/* 姝屾洸淇℃伅閬僵瀹瑰櫒锛氭尐鐫€灏侀潰闈犲乏锛屽崰鎹彸渚у墿浣欑┖闂?*/
.music-info-mask-box {
    position: absolute;
    left: 30px;
    right: 18px;
    height: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
    padding-left: 0;
    -webkit-app-region: no-drag;
    transform: translateY(-1px) translateX(-0.5px);
    mask-image: linear-gradient(to right, #000000 75%, transparent 100%);
    -webkit-mask-image: linear-gradient(to right, #000000 75%, transparent 100%);
}

/* 姝屾洸鏂囨湰鍩虹鏍峰紡 */
.music-info-text {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 12.5px;
    font-weight: 500;
    white-space: nowrap;
    /* 寮哄埗鍗曡涓嶆崲琛?*/
    overflow: hidden;
    color: inherit;
    opacity: 0.9;
}

/* 鐏靛姩宀涙秷鎭€氱煡鏍峰紡 */
.msg-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    padding: 0 45px 0 0px;
    box-sizing: border-box;
    z-index: 10;
    gap: 12px;
    -webkit-app-region: no-drag;
    transition: opacity 0.15s ease;
}

/* F11 閫氱煡鍙偣鍑伙細hover 鏃剁粰涓€鐐硅瑙夊弽棣?*/
.msg-box:hover {
    opacity: 0.8;
}

.msg-box:active {
    opacity: 0.65;
}

/* 棰勫埗娑堟伅鍥炬爣/澶村儚鏍峰紡 */
.msg-avatar {
    width: 35px;
    height: 35px;
    border-radius: 50%;
    background: none;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #ffffff;
    flex-shrink: 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.msg-avatar-img {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    object-fit: cover;
}

/* 鏂囨湰闈犲乏瀵归綈鍖呰９灞?*/
.msg-text-wrapper {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    overflow: hidden;
    flex-grow: 1;
}

/* 娑堟伅寮圭獥瀹瑰櫒 */
.msg-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: 700;
    line-height: 1.4;
    width: 100%;
    overflow: hidden;
}

/* 鍙戦€佽€呮樀绉帮紙鍏佽瓒呴暱鐪佺暐鍙凤級 */
.sender-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

/* 灏鹃儴鐨勭▼搴忓悕 */
.app-name {
    font-size: 10.5px;
    font-weight: 600;
    flex-shrink: 0;
    padding: 2px 6px;
    border-radius: 6px;
    background-color: rgba(150, 150, 150, 0.25);
    color: inherit;
    opacity: 0.9;
    letter-spacing: 0.2px;
    transform: translateY(-0.5px);
}

/* 璋冨ぇ鍚庣殑鍐呭鏍峰紡 */
.msg-body {
    font-size: 12.5px;
    line-height: 1.4;
    opacity: 0.75;
    text-align: left;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.value.high-usage {
    color: #f06861 !important;
}


/* 闊充箰寰嬪姩棰戣氨鏍峰紡 */
.audio-spectrum {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 2px;
    height: 12px;
    padding-right: 2px;
}

/* 鏆傚仠鐘舵€佷笅鐨勭珫绾匡紙缁熶竴楂樺害锛?*/
.audio-spectrum .bar {
    width: 2px;
    height: 18px;
    background-color: #b6e0ee;
    border-radius: 3px;
    transform-origin: center;
    /* 鏀圭敤鏋侀€熺殑 ease-out 杩囨浮锛岃鍓嶇瀹岀編琛旀帴鍚庣鐨勫抚鐜?*/
    transition: transform 0.08s ease-out;
    will-change: transform;
}

.music-ctl-box {
    transition: opacity 0.2s ease !important;
}

.music-ctl-box.expanded {
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    padding: 0 !important;
}

/* 椤堕儴瀹瑰櫒锛氬彇娑?all 杩囨浮锛岃瀹冭窡鐫€ Rust 绐楀彛鐨勬媺浼镐弗涓濆悎缂濆湴閲嶆帓 */
.music-top-row {
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
    position: relative;
    transition: none !important;
    /* 鏍稿績闃叉姈榄旀硶锛屽彇娑?CSS 鐨勬專鎵?*/
}

.music-ctl-box.expanded .music-top-row {
    height: 40px;
    margin-top: 14px !important;
    margin-left: 5px !important;
    border: none;
}

/* 灏侀潰锛氳鐩栨帀涓婇潰鐨?transition: all锛屽彧淇濈暀鍙樺舰鍜屽渾瑙掔殑杩囨浮 */
.album-cover {
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.2), border-radius 0.3s ease !important;
}

.music-ctl-box.expanded .album-cover {
    width: 40px !important;
    height: 40px !important;
    border-radius: 6px !important;
    animation: none !important;
    border: none;
    transform: translateX(0px) rotate(0deg) !important;
}

.music-ctl-box.expanded .album-cover .cover-inner {
    animation: none !important;
    transform: rotate(0deg) !important;
    border: none;
}

.music-ctl-box.expanded .album-cover.is-playing {
    border: none;
    transform: scale(1.05) translateX(0px) rotate(0deg) !important;
}

/* 姝屾洸鏂囨湰閬僵锛氬彇娑堣繃娓★紝闅忕獥鍙ｅぇ灏忕灛闂村彉鍖?*/
.music-ctl-box.expanded .music-info-mask-box {
    left: 60px !important;
    right: 55px !important;
    display: flex !important;
    align-items: center !important;
    justify-content: flex-start !important;
    transition: none !important;
}

/* 浣犵殑涓ゅ鏂囧瓧杩囨浮閫昏緫闈炲父瀹岀編锛屽叏閮ㄤ繚鐣欏師鏍凤紙鍥犱负 opacity 涓嶅奖鍝嶆帓鐗堬級 */
.music-info-text {
    position: absolute;
    left: 0 !important;
    top: 50%;
    width: 100%;
    transform: translateY(-50%);
    transition: opacity 0.3s ease, transform 0.3s ease;
    text-align: left !important;
    display: flex !important;
    flex-direction: column !important;
    align-items: flex-start !important;
}

.double-line {
    opacity: 0;
    pointer-events: none;
    transform: translateY(-30%);
}

.single-line {
    opacity: 1;
    align-items: center;
    text-align: center;
}

.single-line.fade-out {
    opacity: 0;
    pointer-events: none;
    transform: translateY(20%);
}

.double-line.fade-in {
    opacity: 1;
    pointer-events: auto;
    transform: translateY(-50%) !important;
}

.song-title {
    font-size: 15px;
    font-weight: 700;
    margin-bottom: 2px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.2;
    width: 100%;
    text-align: left !important;
}

.song-artist {
    font-size: 12.5px;
    opacity: 0.65;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.2;
    width: 100%;
    text-align: left !important;
}

/* 濯掍綋鎺т欢涓庨璋?*/
.music-ctl-box.expanded .music-controls {
    position: absolute;
    left: 50%;
    transform: translateX(-50%) translateY(5px);
    width: 100%;
    display: flex;
    justify-content: center;
    gap: 20px;
}

.music-ctl-box.expanded .ctl-btn svg {
    width: 22px;
    height: 22px;
}

.music-ctl-box.expanded .play-btn svg {
    width: 28px;
    height: 28px;
}

.audio-spectrum.expanded {
    position: absolute;
    right: 18px !important;
    top: 27px !important;
    transform: scale(1.3);
    /* 鎶?all 鎹㈡垚鍏蜂綋鐨勫睘鎬э紝闃叉鎶栧姩 */
    transition: opacity 0.3s ease, transform 0.3s ease !important;
}

/* F6 闊充箰杩涘害鏉?*/
.music-progress {
    position: absolute;
    left: 16px;
    right: 16px;
    bottom: 10px;
    display: flex;
    flex-direction: column;
    gap: 3px;
    z-index: 2;
}

.progress-time-row {
    display: flex;
    justify-content: space-between;
    font-size: 10.5px;
    opacity: 0.85;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    letter-spacing: 0.3px;
}

.progress-bar {
    position: relative;
    height: 4px;
    background: rgba(128, 128, 128, 0.25);
    border-radius: 2px;
    cursor: pointer;
    touch-action: none;
    user-select: none;
    transition: height 0.15s ease;
}

.progress-bar:hover {
    height: 6px;
}

.progress-bar.disabled {
    cursor: not-allowed;
    opacity: 0.55;
}

.progress-bar.disabled:hover {
    height: 4px;
}

.progress-bar.disabled .progress-thumb {
    display: none;
}

.progress-filled {
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
    background: currentColor;
    border-radius: 2px;
    transition: width 0.1s linear;
}

.progress-filled.dragging {
    transition: none;
}

.progress-thumb {
    position: absolute;
    top: 50%;
    width: 10px;
    height: 10px;
    background: currentColor;
    border-radius: 50%;
    transform: translate(-50%, -50%);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.35);
    transition: width 0.15s ease, height 0.15s ease;
}

.progress-bar:hover .progress-thumb {
    width: 12px;
    height: 12px;
}

.progress-remaining {
    font-size: 9.5px;
    opacity: 0.55;
    text-align: center;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
}

.progress-placeholder {
    min-height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10.5px;
    opacity: 0.55;
    letter-spacing: 0.2px;
}

/* 寮哄埗闈犲乏瀵归綈锛屽共鎺夊師鏈殑 align-items: center銆傚惁鍒欓暱鏂囨湰浼氬悜涓よ竟婧㈠嚭锛屽鑷村紑澶磋瑁?*/
.music-info-text.single-line {
    overflow: visible !important;
    align-items: flex-start !important;
    text-align: left !important;
}

/* 婊氬姩鐨勫唴閮ㄥ鍣?*/
.scroll-inner {
    display: inline-block;
    white-space: nowrap;
    width: max-content;
    flex-shrink: 0;
    backface-visibility: hidden;
    transform: translateZ(0);
    -webkit-font-smoothing: antialiased;
    transform-style: preserve-3d;
}

/* 鎸傝浇鍔ㄧ敾 */
.scroll-inner.is-scrolling {
    animation: scroll-ping-pong var(--scroll-duration) linear infinite alternate;
}

/* 婊氬姩鍔ㄧ敾甯э細鍒╃敤 0-20% 鍜?80-100% 鐨勫尯闂村疄鐜颁袱绔仠鐣?*/
@keyframes scroll-ping-pong {

    0%,
    20% {
        transform: translateX(0);
    }

    80%,
    100% {
        /* JS 閲屽凡缁忔嫾濂戒簡 px 鍗曚綅锛岃繖閲岀洿鎺?-1 涔樿繃鍘诲嵆鍙?*/
        transform: translateX(calc(-1 * var(--scroll-dist)));
    }
}

/* 绯荤粺鎿嶄綔閫氱煡鏍峰紡 */
.system-toast-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    padding-left: 0;
    z-index: 10;
    -webkit-app-region: no-drag;
}

.toast-icon {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transform: translateX(-8px);
}

/* 鐏靛姩宀涢€氱煡 */
.toast-icon.app-icon {
    color: currentColor;
}

/* 绯荤粺閫氱煡浣跨敤璺熼殢瀛椾綋鐨勫師鐢熷姣旇壊 (榛戠櫧) */
.toast-icon.sys-icon {
    color: currentColor;
    opacity: 0.85;
}

.toast-icon svg {
    width: 22px;
    height: 22px;
    display: block;
}

.toast-icon.battery-charge-icon {
    color: #34C759;
}

.toast-icon.battery-low-icon {
    color: #FF3B30;
}

.toast-text {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 12.5px;
    font-weight: 600;
    white-space: nowrap;
    opacity: 0.95;
    transform: translateX(-2px) translateY(-1px);
}

/* 瀹藉害璋冩暣鎵嬫焺鏍峰紡 */
.resize-handle {
    position: absolute;
    top: 0;
    width: 6px;
    height: 100%;
    z-index: 100;
    cursor: ew-resize;
    transition: opacity 0.2s ease, background-color 0.2s ease;
    opacity: 0;
}

.resize-handle:hover {
    opacity: 1;
    background-color: rgba(255, 255, 255, 0.3);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .resize-handle:hover {
    background-color: rgba(0, 0, 0, 0.2);
}

.resize-handle.left {
    left: 0;
    border-radius: 100px 0 0 100px;
}

.resize-handle.right {
    right: 0;
    border-radius: 0 100px 100px 0;
}

/* 灞曞紑鐘舵€佷笅璋冩暣鎵嬫焺鐨勫渾瑙?*/
.island-container:has(.island-core-content[style*="border-radius: 22px"]) .resize-handle {
    border-radius: 0;
}

.island-container:has(.island-core-content[style*="border-radius: 22px"]) .resize-handle.left {
    border-radius: 24px 0 0 24px;
}

.island-container:has(.island-core-content[style*="border-radius: 22px"]) .resize-handle.right {
    border-radius: 0 24px 24px 0;
}

/* 姝ｅ湪璋冩暣鏃剁殑鏍峰紡 */
.resize-handle:active {
    opacity: 1;
    background-color: rgba(255, 255, 255, 0.4);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .resize-handle:active {
    background-color: rgba(0, 0, 0, 0.3);
}

/* 鍏夋爣鏍峰紡 */
.island-core-content.resize-cursor-left {
    cursor: w-resize;
}

.island-core-content.resize-cursor-right {
    cursor: e-resize;
}
</style>
