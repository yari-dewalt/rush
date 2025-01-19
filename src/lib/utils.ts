export function convertBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return '0 B';
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
  const k = 1000;
  
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  const value = bytes / Math.pow(k, i);
  
  return `${value.toFixed(decimals)} ${units[i]}`;
}

export function convertSeconds(seconds: number): string {
  if (seconds <= 0) return '0s';

  const units = [
    { value: 31536000, label: 'y' },
    { value: 86400, label: 'd' },
    { value: 3600, label: 'h' },
    { value: 60, label: 'm' },
    { value: 1, label: 's' }
  ];

  if (seconds < 60) {
    return `${Math.floor(seconds)}s`;
  }

  let remaining = seconds;
  const parts = [];

  for (const unit of units.slice(0, -1)) {
    const count = Math.floor(remaining / unit.value);
    if (count > 0) {
      parts.push(`${count}${unit.label}`);
      remaining %= unit.value;
    }
  }

  return parts.slice(0, 3).join(' ');
}
