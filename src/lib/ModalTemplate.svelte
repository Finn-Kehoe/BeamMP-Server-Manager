<script lang="ts">
    import { fade } from "svelte/transition";

    const handleKeydown = (e) => {
        if (e.key === 'Escape') return onClose();
    };

    export let show = false;
    export let width = "40%";
    export let onClose = () => {};
</script>

<svelte:window on:keydown|once={handleKeydown} />

<!-- svelte-ignore a11y-click-events-have-key-events -->
{#if show}
    <div class="modal-wrapper">
        <div class="modal-background" on:click={onClose}>
            <div class="modal" on:click|stopPropagation transition:fade={{duration: 150}} style:width={width}>
                <slot />
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-background {
        position: fixed;
        inset: 0;
        background: rgba(0,0,0,0.5);
        z-index: 999;
    }
    .modal {
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: #2b2b2b;
        padding: 1.5rem;
        border-radius: 0.5rem;
        z-index: 1000;
        max-height: 85%;
    }
</style>