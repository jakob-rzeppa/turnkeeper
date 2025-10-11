<template>
    <div
        class="fixed top-0 left-0 z-50 h-screen w-80 bg-base-200 shadow-2xl -translate-x-72 hover:translate-x-0 transition-all duration-300 ease-in-out group border-r border-base-300"
    >
        <!-- Tab Handle -->
        <div
            class="absolute -right-12 top-6 bg-base-200 p-3 rounded-r-lg shadow-lg border-r border-t border-b border-base-300 group-hover:bg-primary group-hover:text-primary-content transition-colors"
        >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 6h16M4 12h16M4 18h16"
                ></path>
            </svg>
        </div>

        <!-- Content -->
        <div
            class="w-full h-full p-6 opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-75"
        >
            <div class="text-center">
                <h2 class="text-xl font-bold text-primary">Game Controls</h2>
                <div class="divider divider-primary"></div>
            </div>
            <div class="flex flex-col gap-2 overflow-y-auto">
                <slot />
            </div>
        </div>
    </div>
</template>
