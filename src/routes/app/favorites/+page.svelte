<script>
    import { onMount } from 'svelte';
    import { route } from '../../../scripts/store.js';

    onMount(() => {
        route.set(window.location.pathname);  // Или установите это значение на маршрут текущей страницы
    });
</script>
<div class="w-full h-[80%] overflow-y-scroll box-border">
    <div>
        <h2 class="pl-[4%] pt-[3%] box-border">Любимые</h2>
        <div class="flex justify-around pt-[3%]">
            <div class="bg-[#454545] w-[100px] h-[100px] relative">
                <p class="absolute top-0 left-0">shadowraze</p>
                <svg class="absolute top-9 left-10" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M5 3L19 12L5 21V3Z" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <p class="absolute bottom-0 right-0">astral step</p>
            </div>
            <div class="bg-[#454545] w-[100px] h-[100px] relative">
                <p class="absolute top-0 left-0">shadowraze</p>
                <svg class="absolute top-9 left-10" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M5 3L19 12L5 21V3Z" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <p class="absolute bottom-0 right-0">astral step</p>
            </div>
            <div class="bg-[#454545] w-[100px] h-[100px] relative">
                <p class="absolute top-0 left-0">shadowraze</p>
                <svg class="absolute top-9 left-10" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M5 3L19 12L5 21V3Z" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <p class="absolute bottom-0 right-0">astral step</p>
            </div>
            <div class="bg-[#454545] w-[100px] h-[100px] relative">
                <p class="absolute top-0 left-0">shadowraze</p>
                <svg class="absolute top-9 left-10" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M5 3L19 12L5 21V3Z" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
                <p class="absolute bottom-0 right-0">astral step</p>
            </div>
        </div>
    </div>
</div>