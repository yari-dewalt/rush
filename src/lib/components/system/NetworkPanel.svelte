<script lang="ts">
  import Panel from "./Panel.svelte";
  import NetworkUsageText from "./NetworkUsageText.svelte";
  import { convertBytes } from '$lib/utils.ts';
  import { faNetworkWired } from "@fortawesome/free-solid-svg-icons";
	import type { NetworkData } from "$lib/types";

  interface NetworkPanelProps {
    networkData: NetworkData,
    type: string,
  }

  let { networkData, type }: NetworkPanelProps = $props();
</script>

<Panel icon={faNetworkWired} title="Network" usageValue={0} hoverInfo="test">
    {#if type === 'text'}
      <NetworkUsageText {networkData} />
    {/if}
    {#if type === 'graph'}
      <!-- <NetworkGraph {networkData} /> -->
    {/if}
</Panel>
