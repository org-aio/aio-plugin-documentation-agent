<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, shallowRef } from 'vue'
import request from '@/config/axios'
import { t } from '@/locales'
import {
  CAPTCHA_HEIGHT,
  CAPTCHA_WIDTH,
  createCaptchaSession,
  scaleCaptchaPoint,
  type CaptchaChallenge,
  type CaptchaPoint,
  type CaptchaType
} from './captcha.mjs'

const props = defineProps<{ type: CaptchaType }>()
const emit = defineEmits<{ verified: [proof: string]; cancel: [] }>()
const challenge = shallowRef<CaptchaChallenge | null>(null)
const points = ref<CaptchaPoint[]>([])
const offset = ref(0)
const cursor = ref({ x: CAPTCHA_WIDTH / 2, y: CAPTCHA_HEIGHT / 2 })
const keyboardCursor = ref(false)
const busy = ref(false)
const errorMessage = ref('')
const session = createCaptchaSession({
  type: props.type,
  post: (url, data, signal) => request.postOriginal({ url, data, signal })
})
const canVerify = computed(
  () =>
    challenge.value &&
    (props.type === 'blockPuzzle' || points.value.length === challenge.value.wordList?.length)
)
let operation = 0
let expiryTimer: ReturnType<typeof setTimeout> | undefined

const clearExpiry = () => {
  clearTimeout(expiryTimer)
  expiryTimer = undefined
}
const messageFor = (error: unknown) => {
  const failure = error as { messageKey?: string; serverMessage?: string }
  if (failure?.messageKey) {
    return failure.serverMessage || t(failure.messageKey)
  }
  return error instanceof Error ? error.message : t('account.captchaFailed')
}

const refresh = async (preserveError = false) => {
  const currentOperation = ++operation
  clearExpiry()
  challenge.value = null
  points.value = []
  offset.value = 0
  keyboardCursor.value = false
  if (!preserveError) {
    errorMessage.value = ''
  }
  busy.value = true
  try {
    const result = await session.load()
    if (currentOperation !== operation) {
      return
    }
    challenge.value = result
    expiryTimer = setTimeout(
      () => {
        session.cancel()
        challenge.value = null
        errorMessage.value = t('account.captchaExpired')
      },
      Math.max(0, result.expiresAt - Date.now())
    )
  } catch (error) {
    if (
      currentOperation === operation &&
      !(error instanceof Error && error.name === 'AbortError')
    ) {
      errorMessage.value = messageFor(error)
    }
  } finally {
    if (currentOperation === operation) {
      busy.value = false
    }
  }
}

const verify = async () => {
  if (busy.value || !canVerify.value) {
    return
  }
  const currentOperation = ++operation
  busy.value = true
  clearExpiry()
  try {
    const selected = props.type === 'blockPuzzle' ? { x: offset.value, y: 5 } : points.value
    const proof = await session.verify(selected)
    if (currentOperation === operation) {
      emit('verified', proof)
    }
  } catch (error) {
    if (currentOperation !== operation || (error instanceof Error && error.name === 'AbortError')) {
      return
    }
    errorMessage.value = messageFor(error)
    await refresh(true)
  } finally {
    if (currentOperation === operation) {
      busy.value = false
    }
  }
}

const addPoint = (point: CaptchaPoint) => {
  if (
    busy.value ||
    !challenge.value ||
    points.value.length >= (challenge.value.wordList?.length || 0)
  ) {
    return
  }
  points.value.push(point)
  if (canVerify.value) {
    void verify()
  }
}
const selectPoint = (event: MouseEvent) => {
  if (props.type !== 'clickWord' || event.detail === 0) {
    return
  }
  keyboardCursor.value = false
  const rectangle = (event.currentTarget as HTMLElement).getBoundingClientRect()
  addPoint(
    scaleCaptchaPoint(
      event.clientX - rectangle.left,
      event.clientY - rectangle.top,
      rectangle.width,
      rectangle.height
    )
  )
}
const moveCursor = (event: KeyboardEvent) => {
  if (props.type !== 'clickWord') {
    return
  }
  const movement: Record<string, [number, number]> = {
    ArrowLeft: [-5, 0],
    ArrowRight: [5, 0],
    ArrowUp: [0, -5],
    ArrowDown: [0, 5]
  }
  if (movement[event.key]) {
    event.preventDefault()
    keyboardCursor.value = true
    const [x, y] = movement[event.key]
    cursor.value = scaleCaptchaPoint(
      cursor.value.x + x,
      cursor.value.y + y,
      CAPTCHA_WIDTH,
      CAPTCHA_HEIGHT
    )
  } else if (event.key === 'Enter' || event.key === ' ') {
    event.preventDefault()
    keyboardCursor.value = true
    addPoint({ ...cursor.value })
  }
}
const close = () => {
  operation += 1
  clearExpiry()
  session.cancel()
  emit('cancel')
}

onMounted(() => refresh())
onBeforeUnmount(() => {
  operation += 1
  clearExpiry()
  session.cancel()
})
</script>

<template>
  <el-dialog
    :model-value="true"
    :title="t('account.captchaTitle')"
    width="min(390px, calc(100vw - 32px))"
    align-center
    append-to-body
    :close-on-click-modal="false"
    @update:model-value="close"
  >
    <div class="captcha-challenge" :aria-busy="busy">
      <el-alert
        v-if="errorMessage"
        :title="errorMessage"
        type="error"
        :closable="false"
        role="alert"
      />
      <template v-if="challenge">
        <p class="captcha-instruction" aria-live="polite">
          {{ type === 'blockPuzzle' ? t('account.captchaSlide') : t('account.captchaClick') }}
          <strong v-if="type === 'clickWord'">{{ challenge.wordList?.join('、') }}</strong>
        </p>
        <button
          class="captcha-image"
          type="button"
          :aria-label="t('account.captchaImage')"
          :disabled="busy || type === 'blockPuzzle'"
          @click="selectPoint"
          @keydown="moveCursor"
        >
          <img
            :src="`data:image/png;base64,${challenge.originalImageBase64}`"
            alt=""
            draggable="false"
          />
          <img
            v-if="type === 'blockPuzzle'"
            class="captcha-piece"
            :style="{ left: `${(offset / CAPTCHA_WIDTH) * 100}%` }"
            :src="`data:image/png;base64,${challenge.jigsawImageBase64}`"
            alt=""
            draggable="false"
          />
          <span
            v-for="(point, index) in points"
            :key="index"
            class="captcha-point"
            :style="{
              left: `${(point.x / CAPTCHA_WIDTH) * 100}%`,
              top: `${(point.y / CAPTCHA_HEIGHT) * 100}%`
            }"
            >{{ index + 1 }}</span
          >
          <span
            v-if="keyboardCursor && type === 'clickWord'"
            class="captcha-cursor"
            :style="{
              left: `${(cursor.x / CAPTCHA_WIDTH) * 100}%`,
              top: `${(cursor.y / CAPTCHA_HEIGHT) * 100}%`
            }"
          />
        </button>
        <template v-if="type === 'blockPuzzle'">
          <input
            v-model.number="offset"
            class="captcha-slider"
            type="range"
            min="0"
            :max="CAPTCHA_WIDTH - 47"
            step="1"
            :disabled="busy"
            :aria-label="t('account.captchaSlide')"
            @pointerup="verify"
            @keyup.enter="verify"
          />
          <p class="captcha-hint">{{ t('account.captchaKeyboardSlide') }}</p>
        </template>
        <p v-else class="captcha-hint">{{ t('account.captchaKeyboardPoints') }}</p>
      </template>
      <p v-else-if="busy" class="captcha-loading" role="status">
        {{ t('account.captchaLoading') }}
      </p>
    </div>
    <template #footer>
      <el-button @click="close">{{ t('account.captchaCancel') }}</el-button>
      <el-button :loading="busy" @click="refresh(false)">{{
        t('account.captchaRefresh')
      }}</el-button>
      <el-button type="primary" :disabled="!canVerify" :loading="busy" @click="verify">{{
        t('account.captchaVerify')
      }}</el-button>
    </template>
  </el-dialog>
</template>

<style scoped>
.captcha-challenge {
  width: 100%;
  max-width: 310px;
  margin: auto;
}
.captcha-instruction {
  line-height: 1.6;
  margin: 0 0 12px;
}
.captcha-challenge .el-alert {
  margin-bottom: 12px;
}
.captcha-image {
  position: relative;
  display: block;
  width: 100%;
  padding: 0;
  border: 0;
  aspect-ratio: 2;
  background: var(--el-fill-color);
  cursor: crosshair;
  touch-action: manipulation;
}
.captcha-image > img {
  display: block;
  width: 100%;
  height: 100%;
  user-select: none;
}
.captcha-image > .captcha-piece {
  position: absolute;
  top: 0;
  width: calc(47 / 310 * 100%);
}
.captcha-point,
.captcha-cursor {
  position: absolute;
  transform: translate(-50%, -50%);
  pointer-events: none;
}
.captcha-point {
  display: grid;
  place-items: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: var(--el-color-primary);
  color: var(--el-color-white);
  border: 1px solid var(--el-color-white);
  font-size: 14px;
}
.captcha-cursor {
  width: 14px;
  height: 14px;
  border: 2px solid var(--el-color-white);
  outline: 2px solid var(--el-color-primary);
  border-radius: 50%;
}
.captcha-slider {
  width: 100%;
  height: 40px;
  margin: 12px 0 0;
  accent-color: var(--el-color-primary);
  cursor: grab;
  touch-action: none;
}
.captcha-slider:focus-visible,
.captcha-image:focus-visible {
  outline: 2px solid var(--el-color-primary);
  outline-offset: 3px;
}
.captcha-hint {
  color: var(--el-text-color-secondary);
  font-size: 12px;
  line-height: 1.6;
  margin: 6px 0 0;
}
.captcha-loading {
  min-height: 155px;
  display: grid;
  place-items: center;
  color: var(--el-text-color-secondary);
}
</style>
