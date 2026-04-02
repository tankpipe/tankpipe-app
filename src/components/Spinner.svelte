<script>
    let { show } = $props()
</script>

{#if show}
<div class="spinner">
    <svg role="status" viewBox="0 0 120 30" fill="currentColor">
        <circle cx="15" cy="15" r="15">
            <animate attributeName="r" values="15;9;15" dur="0.8s" repeatCount="indefinite"></animate>
            <animate attributeName="fill-opacity" values="1;.5;1" dur="0.8s" repeatCount="indefinite"></animate>
        </circle>
        <circle cx="60" cy="15" r="9" fill-opacity="0.3">
            <animate attributeName="r" values="9;15;9" dur="0.8s" begin="0.2s" repeatCount="indefinite"></animate>
            <animate attributeName="fill-opacity" values=".5;1;.5" dur="0.8s" begin="0.2s" repeatCount="indefinite"></animate>
        </circle>
        <circle cx="105" cy="15" r="15">
            <animate attributeName="r" values="15;9;15" dur="0.8s" begin="0.4s" repeatCount="indefinite"></animate>
            <animate attributeName="fill-opacity" values="1;.5;1" dur="0.8s" begin="0.4s" repeatCount="indefinite"></animate>
        </circle>
    </svg>
</div>
{/if}
<style>
    .spinner {
        width: 30px;
    }
</style>