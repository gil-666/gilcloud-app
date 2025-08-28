<template>
    <div
        class="background-player-wrapper banner fixed inset-0 place-content-center items-end flex pointer-events-none p-4 z-99">
        <div
            class="background-player-cont justify-between bg-neutral-700 bg-gradient-to-b from-neutral-600 via-neutral-500 to-neutral-600 border-1 border-neutral-500 relative w-full h-14 rounded-4xl pointer-events-auto">
            <div class="inside p-2 w-full h-full flex items-center gap-2">
                <div class="icon min-w-10 flex-col w-12 bg-black rounded-3xl">
                    <img src="../../assets/logtrans.png" alt="Logo" />
                </div>
                <div class="info w-35 flex-col text-center">
                    <p class="font-light">penis nigga</p>
                    <p>penis jr.</p>
                </div>

                <div class="progress-controls flex flex-col w-full items-center gap-2">
                    <div class="controls flex justify-center gap-4 text-lg">
                        <i class="pi pi-step-backward play-button cursor-pointer"></i>
                        <i class="pi pi-play play-button cursor-pointer"></i>
                        <i class="pi pi-step-forward play-button cursor-pointer"></i>
                    </div>
                    <input type="range" class="progress-bar w-full" />
                </div>
                <div>

                </div>
            </div>

        </div>
    </div>

</template>
<script setup lang="ts">

</script>
<style lang="css" scoped>
.progress-bar {
    -webkit-appearance: none;
    width: 100%;
    height: 5px;
    background: #d1d1d1;
    border-radius: 5px;
    outline: none;
}

.progress-bar::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: #ffffff;
    cursor: pointer;
}
</style>