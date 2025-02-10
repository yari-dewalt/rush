<script lang="ts">
  import * as SimpleIcons from 'simple-icons';

  let { processName } = $props<{ processName: string }>();

  function getProcessIcon(name: string): string {
    // TODO: Add more common mappings and improve matching
    // Common process name mappings
    const processMap: Record<string, string> = {
      // Browsers
      'firefox': 'siFirefox',
      'chrome': 'siGooglechrome',
      'chromium': 'siChromium',
      
      // Development
      'code': 'siVisualstudiocode',
      'node': 'siNodedotjs',
      'python': 'siPython',
      
      // System
      'systemd': 'siLinux',
      'bash': 'siGnubash',
      'zsh': 'siGnubash',
      
      // Services
      'docker': 'siDocker',
      'postgres': 'siPostgresql',
      'nginx': 'siNginx',
    };

    // Clean and normalize name
    const cleanName = name.toLowerCase()
      .replace(/[^a-z0-9]/g, '')
      .replace(/(\.exe|\.app)$/, '');

    // Try exact match
    if (processMap[cleanName]) {
      const icon = SimpleIcons[processMap[cleanName] as keyof typeof SimpleIcons];
      return createSvgDataUrl(icon);
    }

    // Try partial match
    const partialMatch = Object.entries(processMap).find(([key]) => 
      cleanName.includes(key));
    if (partialMatch) {
      const icon = SimpleIcons[partialMatch[1] as keyof typeof SimpleIcons];
      return createSvgDataUrl(icon);
    }

    // Fallback to generic process icon
    const formattedName =
      cleanName.charAt(0).toUpperCase() + cleanName.slice(1);
    const iconKey = `si${formattedName}`;
    let simpleIcon = SimpleIcons[iconKey as keyof typeof SimpleIcons];

    // Default icon if no match found
    if (simpleIcon) {
      return createSvgDataUrl(simpleIcon);
    }
    return createSvgDataUrl(SimpleIcons.siLinux);
  }

  function createSvgDataUrl(icon: any): string {
    if (!icon || !icon.svg) return '';
    const svgWithColor = icon.svg.replace("<svg", `<svg fill="#${icon.hex}"`);
    return `data:image/svg+xml;base64,${btoa(svgWithColor)}`;
  }

  const iconUrl = $derived(getProcessIcon(processName));
</script>

<img
  src={iconUrl}
  alt={`${processName} icon`}
  class="w-5 h-5"
/>