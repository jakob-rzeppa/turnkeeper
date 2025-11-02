<script setup lang="ts">
import { ref } from 'vue';
import DisplayContainer from './container/DisplayContainer.vue';

// Range generator state
const min = ref<number>(1);
const max = ref<number>(6);
const rangeResult = ref<number | null>(null);

function rollRange() {
    const a = Math.ceil(Math.min(min.value, max.value));
    const b = Math.floor(Math.max(min.value, max.value));
    // guard against invalid range
    if (b < a) {
        rangeResult.value = null;
        return;
    }
    rangeResult.value = Math.floor(Math.random() * (b - a + 1)) + a;
}

// Percentage event tester state
const percent = ref<number>(50);
const percentResult = ref<{ success: boolean; roll: number } | null>(null);

function testPercent() {
    const p = Math.max(0, Math.min(100, percent.value));
    const roll = Math.random() * 100;
    percentResult.value = { success: roll <= p, roll };
}
</script>

<template>
    <DisplayContainer label="Random Generator">
        <div class="grid gap-4">
            <!-- Range roller -->
            <section class="card bg-base-200 shadow-md">
                <div class="card-body p-4">
                    <h4 class="card-title text-base mb-2">Range Roll</h4>
                    <div class="flex flex-wrap gap-2 items-end">
                        <div class="w-20">
                            <label class="label py-0">
                                <span class="label-text text-xs">Min</span>
                            </label>
                            <input
                                type="number"
                                v-model.number="min"
                                class="input input-sm w-full"
                            />
                        </div>
                        <div class="w-20">
                            <label class="label py-0">
                                <span class="label-text text-xs">Max</span>
                            </label>
                            <input
                                type="number"
                                v-model.number="max"
                                class="input input-sm w-full"
                            />
                        </div>
                        <button @click="rollRange" class="btn btn-primary btn-sm">Roll</button>
                    </div>
                    <div v-if="rangeResult !== null" class="mt-3">
                        <div class="badge badge-lg badge-primary font-bold">
                            Result: {{ rangeResult }}
                        </div>
                    </div>
                </div>
            </section>

            <!-- Percentage tester -->
            <section class="card bg-base-200 shadow-md">
                <div class="card-body p-4">
                    <h4 class="card-title text-base mb-2">Percentage Event</h4>
                    <div class="flex flex-wrap gap-2 items-end">
                        <div class="w-28">
                            <label class="label py-0">
                                <span class="label-text text-xs">Chance (%)</span>
                            </label>
                            <input
                                type="number"
                                v-model.number="percent"
                                min="0"
                                max="100"
                                class="input input-sm w-full"
                            />
                        </div>
                        <button @click="testPercent" class="btn btn-secondary btn-sm">Test</button>
                    </div>
                    <div v-if="percentResult" class="mt-3 space-y-1">
                        <div class="text-sm opacity-70">
                            Roll: {{ percentResult.roll.toFixed(2) }}%
                        </div>
                        <div
                            :class="[
                                'badge badge-lg font-semibold',
                                percentResult.success ? 'badge-success' : 'badge-error',
                            ]"
                        >
                            {{ percentResult.success ? '✓ Success' : '✗ Fail' }}
                        </div>
                    </div>
                </div>
            </section>
        </div>
    </DisplayContainer>
</template>
