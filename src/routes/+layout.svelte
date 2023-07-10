<head>
    <meta charset="UTF-8">
    <title>Kiku</title>
</head>
<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import '/src/app.css';

    let minimizeButton, maximizeButton, closeButton;

    onMount(() => {
        minimizeButton = document.getElementById('minimize-button');
        maximizeButton = document.getElementById('maximize-button');
        closeButton = document.getElementById('close-button');

        minimizeButton && minimizeButton.addEventListener('click', () => {
            invoke('minimize');
        });

        maximizeButton && maximizeButton.addEventListener('click', () => {
            invoke('maximize');
        });

        closeButton && closeButton.addEventListener('click', () => {
            invoke('close');
        });
    });
</script>
<style>
    .cursor-pointer {
        transition: 0.2s;
    }
    .cursor-pointer:hover {
        background-color:  dimgray;
        opacity: 0.7;
        transition: 0.2s;
    }
</style>
<body class="bg-[#111111] text-white h-screen relative overflow-hidden">
    <navbar class="flex justify-end">
        <div data-tauri-drag-region id="drag" class="w-full">

        </div>
        <div id="minimize-button" class="cursor-pointer">
            <svg width="22" height="22" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M3.75 9H14.25" stroke="#454545" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        </div>
        <div id="maximize-button" class="cursor-pointer">
            <svg width="22" height="22" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M11.25 5.625H13.5V7.875" stroke="#454545" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M6.75 12.375H4.5V10.125" stroke="#454545" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M15.1875 3.375H2.8125C2.50184 3.375 2.25 3.62684 2.25 3.9375V14.0625C2.25 14.3732 2.50184 14.625 2.8125 14.625H15.1875C15.4982 14.625 15.75 14.3732 15.75 14.0625V3.9375C15.75 3.62684 15.4982 3.375 15.1875 3.375Z" stroke="#454545" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        </div>
        <div id="close-button" class="cursor-pointer">
            <svg width="22" height="22" viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M13.5 4.5L4.5 13.5" stroke="#773C3C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M4.5 4.5L13.5 13.5" stroke="#773C3C" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        </div>
    </navbar>
    <slot/>
</body>