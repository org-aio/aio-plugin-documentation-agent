<script setup lang="ts">
import { computed } from 'vue'
import { Moon, Sunny } from '@element-plus/icons-vue'
import { resolveAssetUrl, settings } from '@/config'
import { t } from '@/locales'
import type { LoginConfiguration } from './config.mjs'

const props = withDefaults(
  defineProps<{ configuration: LoginConfiguration; embedded?: boolean }>(),
  { embedded: false }
)
const colors = computed(() => {
  const color = props.configuration.backgroundColor
  const channels = [1, 3, 5].map((offset) => Number.parseInt(color.slice(offset, offset + 2), 16))
  const light =
    props.configuration.backgroundType !== 'image' &&
    channels[0] * 0.299 + channels[1] * 0.587 + channels[2] * 0.114 > 165
  return {
    '--login-base': color,
    '--login-overlay': String(props.configuration.backgroundOverlay),
    '--login-hero-ink': light ? '#15213a' : '#f4f8ff',
    '--login-hero-muted': light ? '#40506b' : '#c3d0e4',
    '--login-hero-line': light ? '#15213a20' : '#ffffff1c'
  }
})
</script>

<template>
  <component
    :is="embedded ? 'section' : 'main'"
    class="login-presentation"
    :class="[
      `login-layout-${configuration.layout}`,
      `login-panel-${configuration.panelPosition}`,
      `login-background-${configuration.backgroundType}`,
      { 'login-without-hero': !configuration.showHero, 'login-embedded': embedded }
    ]"
    :style="colors"
  >
    <img
      v-if="configuration.backgroundType === 'image' && configuration.backgroundImage"
      class="login-backdrop"
      :src="resolveAssetUrl(configuration.backgroundImage)"
      alt=""
    />
    <div
      v-if="configuration.backgroundType === 'image'"
      class="login-backdrop-overlay"
      aria-hidden="true"
    />
    <div class="login-stage">
      <section v-if="configuration.showHero" class="login-hero">
        <div v-if="configuration.showBrand" class="login-brand">
          <img
            v-if="configuration.brandImage || settings.logo"
            :src="resolveAssetUrl(configuration.brandImage || settings.logo)"
            alt=""
          />
          <span>{{ settings.title }}</span>
        </div>
        <div class="login-hero-content">
          <slot name="hero" :configuration="configuration">
            <div class="login-hero-rule" aria-hidden="true" />
            <h1 v-if="configuration.heroTitle">{{ configuration.heroTitle }}</h1>
            <p v-if="configuration.heroDescription" class="login-hero-description">
              {{ configuration.heroDescription }}
            </p>
            <div
              v-if="configuration.heroImage || configuration.layout === 'split'"
              class="login-hero-media"
              :class="{ 'is-custom': configuration.heroImage }"
            >
              <img
                v-if="configuration.heroImage"
                :src="resolveAssetUrl(configuration.heroImage)"
                alt=""
              />
              <div v-else class="login-illustration" aria-hidden="true">
                <div class="login-orbit orbit-one" />
                <div class="login-orbit orbit-two" />
                <div class="login-art-dots" />
                <svg class="login-art-window" viewBox="0 0 480 300" fill="none">
                  <rect
                    x="25"
                    y="35"
                    width="406"
                    height="234"
                    rx="18"
                    fill="currentColor"
                    fill-opacity=".09"
                    stroke="currentColor"
                    stroke-opacity=".3"
                  />
                  <path d="M25 76H431" stroke="currentColor" stroke-opacity=".2" />
                  <circle cx="48" cy="56" r="4" fill="currentColor" fill-opacity=".65" />
                  <circle cx="62" cy="56" r="4" fill="currentColor" fill-opacity=".4" />
                  <circle cx="76" cy="56" r="4" fill="currentColor" fill-opacity=".2" />
                  <rect
                    x="47"
                    y="98"
                    width="75"
                    height="146"
                    rx="8"
                    fill="currentColor"
                    fill-opacity=".07"
                  />
                  <path
                    d="M63 119H104M63 139H95M63 160H101M63 180H88"
                    stroke="currentColor"
                    stroke-width="5"
                    stroke-linecap="round"
                    stroke-opacity=".28"
                  />
                  <rect
                    x="143"
                    y="98"
                    width="120"
                    height="53"
                    rx="8"
                    fill="currentColor"
                    fill-opacity=".1"
                  />
                  <rect
                    x="278"
                    y="98"
                    width="130"
                    height="53"
                    rx="8"
                    fill="currentColor"
                    fill-opacity=".07"
                  />
                  <path
                    d="M160 119H197M160 134H235M295 119H340M295 134H376"
                    stroke="currentColor"
                    stroke-width="5"
                    stroke-linecap="round"
                    stroke-opacity=".45"
                  />
                  <rect
                    x="143"
                    y="166"
                    width="265"
                    height="78"
                    rx="8"
                    fill="currentColor"
                    fill-opacity=".06"
                  />
                  <path
                    d="M158 225L195 214L226 219L259 193L289 201L321 180L351 187L392 176"
                    stroke="#64d8d0"
                    stroke-width="4"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <circle cx="392" cy="176" r="5" fill="#64d8d0" />
                </svg>
                <div class="login-floating-tile"><span /><span /><span /></div>
                <div class="login-floating-check">
                  <svg viewBox="0 0 24 24" fill="none">
                    <path
                      d="m6 12 4 4 8-9"
                      stroke="currentColor"
                      stroke-width="2.4"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    />
                  </svg>
                </div>
              </div>
            </div>
          </slot>
        </div>
      </section>
      <section class="login-form-panel">
        <div class="login-page-actions">
          <ElButton
            v-if="configuration.showThemeToggle"
            :icon="settings.dark ? Sunny : Moon"
            circle
            :disabled="embedded"
            :aria-label="t(settings.dark ? 'app.lightMode' : 'app.darkMode')"
            :title="t(settings.dark ? 'app.lightMode' : 'app.darkMode')"
            @click="settings.dark = !settings.dark"
          />
          <slot name="actions" />
        </div>
        <div class="login-form-content">
          <div
            v-if="configuration.showBrand && !configuration.showHero"
            class="login-brand login-form-brand"
          >
            <img
              v-if="configuration.brandImage || settings.logo"
              :src="resolveAssetUrl(configuration.brandImage || settings.logo)"
              alt=""
            /><span>{{ settings.title }}</span>
          </div>
          <header class="login-form-heading">
            <h2 v-if="configuration.formTitle">{{ configuration.formTitle }}</h2>
            <p v-if="configuration.formDescription">{{ configuration.formDescription }}</p>
          </header>
          <slot />
        </div>
        <footer v-if="configuration.showFooter" class="login-footer">
          <slot name="footer" :configuration="configuration">{{ configuration.footer }}</slot>
        </footer>
        <aside
          v-if="configuration.showQrcode && configuration.qrcodeImage"
          class="login-qrcode"
          :aria-label="configuration.qrcodeLabel || t('login.qrcode')"
        >
          <component
            :is="configuration.qrcodeLink ? 'a' : 'div'"
            class="login-qrcode-card"
            :href="configuration.qrcodeLink || undefined"
            target="_blank"
            rel="noreferrer"
          >
            <img :src="resolveAssetUrl(configuration.qrcodeImage)" alt="" />
            <span>{{ configuration.qrcodeLabel || t('login.qrcode') }}</span>
            <small v-if="configuration.qrcodeDescription">{{
              configuration.qrcodeDescription
            }}</small>
          </component>
        </aside>
      </section>
    </div>
  </component>
</template>

<style scoped>
.login-presentation {
  position: relative;
  min-height: 100dvh;
  isolation: isolate;
  background: var(--login-base);
  color: var(--el-text-color-primary);
}
.login-background-gradient {
  background-image: radial-gradient(
      ellipse at 15% 15%,
      color-mix(in srgb, var(--el-color-primary) 38%, transparent),
      transparent 60%
    ),
    radial-gradient(ellipse at 55% 100%, #24b5ad20, transparent 48%);
}
.login-backdrop,
.login-backdrop-overlay {
  position: absolute;
  z-index: -1;
  inset: 0;
  width: 100%;
  height: 100%;
}
.login-backdrop {
  object-fit: cover;
}
.login-backdrop-overlay {
  background: rgb(3 9 23 / var(--login-overlay));
}
.login-stage {
  display: grid;
  grid-template-columns: minmax(0, 1.1fr) minmax(400px, 0.9fr);
  min-height: inherit;
}
.login-hero {
  display: flex;
  flex-direction: column;
  min-width: 0;
  padding: 42px clamp(36px, 5vw, 90px);
  color: var(--login-hero-ink);
}
.login-brand {
  display: flex;
  align-items: center;
  gap: 13px;
  font-size: 20px;
  font-weight: 650;
  letter-spacing: 0.3px;
  overflow-wrap: anywhere;
}
.login-brand img {
  width: 42px;
  height: 42px;
  object-fit: contain;
  flex-shrink: 0;
}
.login-hero-content {
  width: min(100%, 600px);
  margin: auto;
  padding: 64px 0 38px;
}
.login-hero-rule {
  height: 4px;
  width: 44px;
  margin-bottom: 25px;
  border-radius: 4px;
  background: var(--el-color-primary);
}
.login-hero h1 {
  font-size: clamp(34px, 3.4vw, 54px);
  line-height: 1.25;
  font-weight: 650;
  letter-spacing: 0.02em;
  white-space: pre-line;
  overflow-wrap: anywhere;
}
.login-hero-description {
  max-width: 440px;
  margin-top: 21px;
  line-height: 1.9;
  font-size: 16px;
  color: var(--login-hero-muted);
  white-space: pre-line;
}
.login-hero-media {
  height: clamp(220px, 27vw, 360px);
  margin-top: 36px;
}
.login-hero-media > img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
.login-illustration {
  position: relative;
  width: 100%;
  height: 100%;
  max-width: 500px;
}
.login-art-window {
  position: absolute;
  inset: 4% 0 0;
  width: 100%;
  height: 92%;
  transform: rotate(-4deg);
  filter: drop-shadow(0 20px 25px #00000012);
}
.login-orbit {
  position: absolute;
  border: 1px solid var(--login-hero-line);
  border-radius: 50%;
}
.orbit-one {
  width: 62%;
  aspect-ratio: 1;
  top: -2%;
  left: 20%;
}
.orbit-two {
  width: 77%;
  aspect-ratio: 1;
  top: -13%;
  left: 12%;
  border-style: dashed;
  opacity: 0.7;
}
.login-art-dots {
  position: absolute;
  width: 92px;
  height: 92px;
  top: 0;
  right: 2%;
  background-image: radial-gradient(var(--login-hero-line) 2px, transparent 2px);
  background-size: 14px 14px;
}
.login-floating-tile {
  position: absolute;
  display: flex;
  align-items: end;
  gap: 7px;
  padding: 18px;
  left: 1%;
  bottom: 7%;
  width: 90px;
  height: 76px;
  border-radius: 14px;
  background: #3766ba;
  box-shadow: 0 15px 35px #07152e30;
  transform: rotate(-8deg);
  animation: login-float 6s ease-in-out infinite;
}
.login-floating-tile span {
  flex: 1;
  height: 45%;
  border-radius: 3px;
  background: #b4d7ff;
}
.login-floating-tile span:nth-child(2) {
  height: 95%;
  background: #fff;
}
.login-floating-tile span:nth-child(3) {
  height: 70%;
  background: #72bbea;
}
.login-floating-check {
  position: absolute;
  display: grid;
  place-items: center;
  right: 5%;
  top: 2%;
  width: 60px;
  height: 60px;
  border-radius: 17px;
  background: #dff8f2;
  color: #258b7b;
  box-shadow: 0 15px 35px #07152e20;
  transform: rotate(8deg);
  animation: login-float 7s ease-in-out infinite reverse;
}
.login-floating-check svg {
  width: 31px;
}
.login-form-panel {
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-width: 0;
  padding: 110px clamp(34px, 5vw, 90px) 70px;
  background: var(--el-bg-color);
  border-left: 1px solid var(--panel-border-color);
}
.login-form-content {
  width: min(100%, 410px);
  margin: auto;
}
.login-form-heading {
  margin-bottom: 34px;
}
.login-form-heading h2 {
  font-size: 30px;
  line-height: 1.3;
  font-weight: 650;
  letter-spacing: 0.5px;
  overflow-wrap: anywhere;
}
.login-form-heading p {
  margin-top: 12px;
  font-size: 14px;
  line-height: 1.8;
  color: var(--el-text-color-secondary);
  white-space: pre-line;
}
.login-form-brand {
  margin-bottom: 40px;
}
.login-page-actions {
  position: absolute;
  top: 28px;
  right: 30px;
  display: flex;
  align-items: center;
  gap: 8px;
}
.login-page-actions :deep(.el-button + .el-button) {
  margin-left: 0;
}
.login-footer {
  margin-top: 45px;
  text-align: center;
  color: var(--el-text-color-secondary);
  font-size: 12px;
  line-height: 1.8;
  white-space: pre-line;
}
.login-qrcode {
  position: fixed;
  right: 26px;
  bottom: 24px;
  z-index: 2;
}
.login-qrcode-card {
  display: grid;
  justify-items: center;
  gap: 5px;
  width: 108px;
  padding: 8px;
  border: 1px solid var(--panel-border-color);
  border-radius: 10px;
  background: color-mix(in srgb, var(--el-bg-color) 90%, transparent);
  color: var(--el-text-color-secondary);
  text-align: center;
  text-decoration: none;
  box-shadow: 0 14px 35px #040c2212;
}
.login-qrcode-card img {
  width: 86px;
  height: 86px;
  object-fit: contain;
}
.login-qrcode-card span {
  font-size: 12px;
  line-height: 1.3;
}
.login-qrcode-card small {
  font-size: 10px;
  line-height: 1.35;
  white-space: pre-line;
}
.login-panel-left.login-layout-split .login-hero {
  order: 2;
}
.login-panel-left.login-layout-split .login-form-panel {
  order: 1;
  border-left: 0;
  border-right: 1px solid var(--panel-border-color);
}
.login-panel-left.login-layout-split .login-stage {
  grid-template-columns: minmax(400px, 0.9fr) minmax(0, 1.1fr);
}
.login-layout-centered .login-stage,
.login-without-hero .login-stage {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 28px;
  padding: 40px 24px;
}
.login-layout-centered .login-hero {
  width: min(100%, 540px);
  padding: 0;
  text-align: center;
}
.login-layout-centered .login-brand {
  justify-content: center;
  margin-bottom: 28px;
}
.login-layout-centered .login-hero-content {
  padding: 0;
}
.login-layout-centered .login-hero-rule {
  display: none;
}
.login-layout-centered .login-hero h1 {
  font-size: 30px;
}
.login-layout-centered .login-hero-description {
  margin: 12px auto 0;
  font-size: 14px;
}
.login-layout-centered .login-hero-media {
  height: 130px;
  margin-top: 18px;
}
.login-layout-centered .login-form-panel,
.login-without-hero .login-form-panel {
  width: min(100%, 500px);
  padding: 64px 40px 38px;
  border: 1px solid var(--panel-border-color);
  border-radius: 22px;
  box-shadow: 0 22px 70px #040c221c;
  order: 1;
}
.login-layout-centered .login-form-heading h2,
.login-without-hero .login-form-heading h2 {
  font-size: 27px;
}
.login-layout-centered .login-page-actions,
.login-without-hero .login-page-actions {
  top: 17px;
  right: 20px;
}
.login-embedded {
  min-height: 570px;
}
.login-embedded .login-hero {
  padding: 32px;
}
.login-embedded .login-hero-content {
  padding: 36px 0 20px;
}
.login-embedded .login-hero h1 {
  font-size: 32px;
}
.login-embedded .login-hero-media {
  height: 210px;
  margin-top: 24px;
}
.login-embedded .login-form-panel {
  padding: 75px 34px 36px;
}
.login-embedded .login-form-heading {
  margin-bottom: 25px;
}
.login-embedded.login-layout-centered .login-hero {
  padding: 0;
}
.login-embedded.login-layout-centered .login-hero-content {
  padding: 0;
}
@keyframes login-float {
  50% {
    translate: 0 -8px;
  }
}
@media (max-width: 820px) {
  .login-stage,
  .login-panel-left.login-layout-split .login-stage {
    display: flex;
    flex-direction: column;
  }
  .login-hero,
  .login-panel-left.login-layout-split .login-hero {
    order: 0;
    padding: 25px 25px 28px;
  }
  .login-brand {
    font-size: 17px;
    gap: 10px;
  }
  .login-brand img {
    width: 34px;
    height: 34px;
  }
  .login-hero-content {
    margin: 28px 0 0;
    padding: 0;
  }
  .login-hero h1 {
    font-size: 29px;
  }
  .login-hero-rule {
    display: none;
  }
  .login-hero-description {
    margin-top: 10px;
    font-size: 14px;
    line-height: 1.7;
  }
  .login-hero-media:not(.is-custom) {
    display: none;
  }
  .login-hero-media.is-custom {
    height: 130px;
    margin-top: 20px;
  }
  .login-form-panel,
  .login-panel-left.login-layout-split .login-form-panel {
    order: 1;
    flex: 1;
    padding: 73px 25px 28px;
    border: 0;
    border-radius: 22px 22px 0 0;
  }
  .login-form-content {
    margin: 0 auto auto;
  }
  .login-page-actions {
    top: 20px;
    right: 23px;
  }
  .login-form-heading h2 {
    font-size: 27px;
  }
  .login-form-heading {
    margin-bottom: 26px;
  }
  .login-layout-centered .login-stage,
  .login-without-hero .login-stage {
    padding: 24px 16px;
  }
  .login-layout-centered .login-form-panel,
  .login-without-hero .login-form-panel {
    padding: 67px 25px 28px;
    border-radius: 18px;
  }
  .login-layout-centered .login-hero-content {
    margin: 0;
  }
  .login-footer {
    margin-top: 30px;
  }
  .login-qrcode {
    position: static;
    display: flex;
    justify-content: center;
    margin-top: 18px;
  }
  .login-qrcode-card {
    width: 96px;
  }
  .login-qrcode-card img {
    width: 76px;
    height: 76px;
  }
}
@media (prefers-reduced-motion: reduce) {
  .login-floating-tile,
  .login-floating-check {
    animation: none;
  }
}
</style>
