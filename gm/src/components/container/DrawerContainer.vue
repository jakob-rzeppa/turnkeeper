<template>
    <div
        class="fixed top-0 left-0 z-50 h-screen w-2xs bg-base-200 -translate-x-5/6 hover:translate-x-0 transition-transform shadow-lg group"
    >
        <div
            class="w-full h-full p-4 flex flex-col gap-4 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
        >
            <slot />
        </div>
    </div>
</template>
