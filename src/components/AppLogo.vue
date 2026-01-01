<template>
  <svg width="60" height="60" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg" class="app-logo">
    <defs>
      <linearGradient id="logoGradient" x1="0" y1="0" x2="100" y2="100">
        <stop offset="0%" stop-color="#818cf8" />
        <stop offset="100%" stop-color="#34d399" />
      </linearGradient>
      <filter id="glow">
        <feGaussianBlur stdDeviation="2.5" result="coloredBlur"/>
        <feMerge>
          <feMergeNode in="coloredBlur"/>
          <feMergeNode in="SourceGraphic"/>
        </feMerge>
      </filter>
    </defs>
    
    <!-- Geometric Star Base -->
    <path d="M50 0 L63 37 L100 50 L63 63 L50 100 L37 63 L0 50 L37 37 Z" fill="url(#logoGradient)" opacity="0.2" />
    
    <!-- Inner Octagon/Star -->
    <path d="M50 15 L59 41 L85 50 L59 59 L50 85 L41 59 L15 50 L41 41 Z" stroke="url(#logoGradient)" stroke-width="3" fill="none" filter="url(#glow)" />
    
    <!-- Central Beam/Eye -->
    <circle cx="50" cy="50" r="8" fill="#fbbf24" />
  </svg>
</template>

<style scoped>
.app-logo {
  filter: drop-shadow(0 0 10px rgba(129, 140, 248, 0.4));
  animation: float 6s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0) rotate(0deg); }
  50% { transform: translateY(-5px) rotate(5deg); }
}
</style>
