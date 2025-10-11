<script setup lang="ts"></script>

<template>
    <div class="min-h-screen p-4 w-full mx-auto">
        <div class="card bg-base-100 shadow-xl">
            <div class="card-body">
                <slot />
            </div>
        </div>
    </div>
    <!-- Footer spacer for mobile -->
    <div class="h-20"></div>
</template>
