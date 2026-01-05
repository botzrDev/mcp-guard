import { Component, OnInit, OnDestroy, signal, ChangeDetectionStrategy, ElementRef, ViewChild, AfterViewInit, PLATFORM_ID, inject } from '@angular/core';
import { CommonModule, isPlatformBrowser } from '@angular/common';

interface Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  opacity: number;
  type: 'shield' | 'dot' | 'hex';
  rotation: number;
  rotationSpeed: number;
}

interface Node {
  x: number;
  y: number;
  active: boolean;
  pulsePhase: number;
}

interface DataPacket {
  connectionIdx: number;
  progress: number; // 0 to 1
  speed: number;
  reverse: boolean; // travel direction
}

@Component({
  selector: 'app-background',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <div class="background-container" [class.reduced-motion]="prefersReducedMotion()">
      <!-- Layer 1: Hexagonal Grid Pattern -->
      <div class="hex-grid-layer"></div>

      <!-- Layer 2: Gradient Glow Orbs -->
      <div class="glow-orbs">
        <div class="glow-orb orb-1"></div>
        <div class="glow-orb orb-2"></div>
        <div class="glow-orb orb-3"></div>
      </div>

      <!-- Layer 3: Node Network (SVG) -->
      <svg class="node-network" viewBox="0 0 1920 1080" preserveAspectRatio="xMidYMid slice">
        <defs>
          <linearGradient id="line-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
            <stop offset="0%" stop-color="#FF7A30" stop-opacity="0"/>
            <stop offset="50%" stop-color="#FF7A30" stop-opacity="0.3"/>
            <stop offset="100%" stop-color="#FF7A30" stop-opacity="0"/>
          </linearGradient>
          <filter id="glow">
            <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
            <feMerge>
              <feMergeNode in="coloredBlur"/>
              <feMergeNode in="SourceGraphic"/>
            </feMerge>
          </filter>
        </defs>

        <!-- Connection lines between nodes -->
        @for (connection of connections(); track $index) {
          <line
            [attr.x1]="connection.x1"
            [attr.y1]="connection.y1"
            [attr.x2]="connection.x2"
            [attr.y2]="connection.y2"
            stroke="url(#line-gradient)"
            stroke-width="1"
            class="connection-line"
            [style.animation-delay]="$index * 0.5 + 's'"
          />
        }

        <!-- Network nodes -->
        @for (node of nodes(); track $index) {
          <g class="network-node" [class.active]="node.active" [style.--pulse-delay]="node.pulsePhase + 's'">
            <circle
              [attr.cx]="node.x"
              [attr.cy]="node.y"
              r="4"
              fill="#FF7A30"
              [attr.opacity]="node.active ? 0.8 : 0.3"
              filter="url(#glow)"
            />
            @if (node.active) {
              <circle
                [attr.cx]="node.x"
                [attr.cy]="node.y"
                r="12"
                fill="none"
                stroke="#FF7A30"
                stroke-width="1"
                class="pulse-ring"
              />
            }
          </g>
        }

      </svg>

      <!-- Layer 4: Canvas for Particles -->
      <canvas #particleCanvas class="particle-canvas"></canvas>

      <!-- Layer 5: Floating Shield Icons (CSS) -->
      <div class="floating-shields">
        @for (shield of floatingShields(); track $index) {
          <div
            class="floating-shield"
            [style.left.%]="shield.x"
            [style.animation-duration]="shield.duration + 's'"
            [style.animation-delay]="shield.delay + 's'"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
              <path d="M12 2L4 6v6c0 5.25 3.4 10.15 8 11.5 4.6-1.35 8-6.25 8-11.5V6l-8-4z"/>
            </svg>
          </div>
        }
      </div>

      <!-- Layer 6: Scan Lines (subtle) -->
      <div class="scan-lines"></div>
    </div>
  `,
  styles: [`
    .background-container {
      position: fixed;
      top: 0;
      left: 0;
      width: 100vw;
      height: 100vh;
      pointer-events: none;
      z-index: 0;
      overflow: hidden;
    }

    /* Layer 1: Hexagonal Grid */
    .hex-grid-layer {
      position: absolute;
      inset: 0;
      background-image: url('/assets/svg/hex-pattern.svg');
      background-size: 60px 69px;
      opacity: 0.4;
      animation: hex-drift 60s linear infinite;
    }

    /* Layer 2: Glow Orbs */
    .glow-orbs {
      position: absolute;
      inset: 0;
    }

    .glow-orb {
      position: absolute;
      border-radius: 50%;
      filter: blur(80px);
      animation: orb-float 20s ease-in-out infinite;
    }

    .orb-1 {
      width: 600px;
      height: 600px;
      background: radial-gradient(circle, rgba(255, 122, 48, 0.15) 0%, transparent 70%);
      top: -200px;
      right: -100px;
      animation-delay: 0s;
    }

    .orb-2 {
      width: 500px;
      height: 500px;
      background: radial-gradient(circle, rgba(59, 130, 246, 0.1) 0%, transparent 70%);
      bottom: -150px;
      left: -100px;
      animation-delay: -7s;
    }

    .orb-3 {
      width: 400px;
      height: 400px;
      background: radial-gradient(circle, rgba(59, 130, 246, 0.08) 0%, transparent 70%);
      top: 40%;
      left: 30%;
      animation-delay: -14s;
    }

    /* Layer 3: Node Network */
    .node-network {
      position: absolute;
      inset: 0;
      width: 100%;
      height: 100%;
    }

    .connection-line {
      stroke-dasharray: 8 4;
      animation: data-flow 3s linear infinite;
    }

    .network-node {
      transition: opacity 0.5s ease;
    }

    .pulse-ring {
      animation: node-pulse 2s ease-out infinite;
      animation-delay: var(--pulse-delay, 0s);
    }

    /* Layer 4: Particle Canvas */
    .particle-canvas {
      position: absolute;
      inset: 0;
      width: 100%;
      height: 100%;
    }

    /* Layer 5: Floating Shields */
    .floating-shields {
      position: absolute;
      inset: 0;
    }

    .floating-shield {
      position: absolute;
      bottom: -50px;
      width: 24px;
      height: 24px;
      color: rgba(255, 122, 48, 0.3);
      animation: shield-rise 20s linear infinite;

      svg {
        width: 100%;
        height: 100%;
      }
    }

    /* Layer 6: Scan Lines */
    .scan-lines {
      position: absolute;
      inset: 0;
      background: repeating-linear-gradient(
        0deg,
        transparent,
        transparent 2px,
        rgba(255, 122, 48, 0.01) 2px,
        rgba(255, 122, 48, 0.01) 4px
      );
      animation: scan-drift 8s linear infinite;
    }

    /* Reduced Motion */
    .reduced-motion {
      .hex-grid-layer,
      .glow-orb,
      .connection-line,
      .pulse-ring,
      .floating-shield,
      .scan-lines {
        animation: none !important;
      }

      .hex-grid-layer {
        opacity: 0.3;
      }

      .floating-shields {
        display: none;
      }
    }

    /* Animations */
    @keyframes hex-drift {
      0% { background-position: 0 0; }
      100% { background-position: 60px 69px; }
    }

    @keyframes orb-float {
      0%, 100% { transform: translate(0, 0) scale(1); opacity: 1; }
      25% { transform: translate(30px, -20px) scale(1.05); }
      50% { transform: translate(-20px, 30px) scale(0.95); opacity: 0.8; }
      75% { transform: translate(20px, 20px) scale(1.02); }
    }

    @keyframes data-flow {
      0% { stroke-dashoffset: 24; }
      100% { stroke-dashoffset: 0; }
    }

    @keyframes node-pulse {
      0% {
        r: 4;
        opacity: 0.8;
      }
      100% {
        r: 24;
        opacity: 0;
      }
    }

    @keyframes shield-rise {
      0% {
        transform: translateY(0) rotate(0deg) scale(0.8);
        opacity: 0;
      }
      10% {
        opacity: 0.4;
        transform: translateY(-10vh) rotate(10deg) scale(1);
      }
      90% {
        opacity: 0.4;
        transform: translateY(-100vh) rotate(-10deg) scale(1);
      }
      100% {
        transform: translateY(-110vh) rotate(0deg) scale(0.8);
        opacity: 0;
      }
    }

    @keyframes scan-drift {
      0% { transform: translateY(0); }
      100% { transform: translateY(4px); }
    }
  `]
})
export class BackgroundComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('particleCanvas') canvasRef!: ElementRef<HTMLCanvasElement>;

  private platformId = inject(PLATFORM_ID);
  private animationFrameId: number | null = null;
  private particles: Particle[] = [];
  private packets: DataPacket[] = [];
  private ctx: CanvasRenderingContext2D | null = null;
  private nodeToggleInterval: ReturnType<typeof setInterval> | null = null;
  private isMobile = false;

  // Animation constants
  private readonly PACKET_COUNT = 8;
  private readonly PACKET_COUNT_MOBILE = 4;
  private readonly ATTRACTION_STRENGTH = 0.00008;
  private readonly ATTRACTION_STRENGTH_MOBILE = 0.00004;

  prefersReducedMotion = signal(false);
  nodes = signal<Node[]>([]);
  connections = signal<{ x1: number; y1: number; x2: number; y2: number }[]>([]);
  floatingShields = signal<{ x: number; duration: number; delay: number }[]>([]);

  ngOnInit() {
    if (isPlatformBrowser(this.platformId)) {
      // Check for reduced motion preference
      const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
      this.prefersReducedMotion.set(mediaQuery.matches);
      mediaQuery.addEventListener('change', (e) => this.prefersReducedMotion.set(e.matches));

      // Detect mobile
      this.isMobile = window.innerWidth < 768;

      // Generate network nodes
      this.generateNodes();

      // Generate floating shields
      this.generateFloatingShields();
    }
  }

  ngAfterViewInit() {
    if (isPlatformBrowser(this.platformId) && !this.prefersReducedMotion()) {
      this.initCanvas();
      this.generateParticles();
      this.generatePackets();
      this.animate();
    }
  }

  ngOnDestroy() {
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId);
    }
    if (this.nodeToggleInterval) {
      clearInterval(this.nodeToggleInterval);
    }
  }

  private generateNodes() {
    const nodeCount = 12;
    const newNodes: Node[] = [];
    const newConnections: { x1: number; y1: number; x2: number; y2: number }[] = [];

    // Generate nodes in a distributed pattern
    for (let i = 0; i < nodeCount; i++) {
      newNodes.push({
        x: 100 + Math.random() * 1720,
        y: 100 + Math.random() * 880,
        active: Math.random() > 0.6,
        pulsePhase: Math.random() * 2
      });
    }

    // Connect nearby nodes
    for (let i = 0; i < newNodes.length; i++) {
      for (let j = i + 1; j < newNodes.length; j++) {
        const dx = newNodes[j].x - newNodes[i].x;
        const dy = newNodes[j].y - newNodes[i].y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < 400 && Math.random() > 0.5) {
          newConnections.push({
            x1: newNodes[i].x,
            y1: newNodes[i].y,
            x2: newNodes[j].x,
            y2: newNodes[j].y
          });
        }
      }
    }

    this.nodes.set(newNodes);
    this.connections.set(newConnections);

    // Periodically toggle node active states
    if (isPlatformBrowser(this.platformId)) {
      this.nodeToggleInterval = setInterval(() => {
        const current = this.nodes();
        const updated = current.map(node => ({
          ...node,
          active: Math.random() > 0.65
        }));
        this.nodes.set(updated);
      }, 3000);
    }
  }

  private generateFloatingShields() {
    const shields: { x: number; duration: number; delay: number }[] = [];
    const count = 8;

    for (let i = 0; i < count; i++) {
      shields.push({
        x: 5 + Math.random() * 90,
        duration: 15 + Math.random() * 10,
        delay: Math.random() * 15
      });
    }

    this.floatingShields.set(shields);
  }

  private initCanvas() {
    const canvas = this.canvasRef.nativeElement;
    this.ctx = canvas.getContext('2d');

    const resize = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    };

    resize();
    window.addEventListener('resize', resize);
  }

  private generateParticles() {
    const particleCount = 15;

    for (let i = 0; i < particleCount; i++) {
      this.particles.push(this.createParticle());
    }
  }

  private createParticle(): Particle {
    const types: ('shield' | 'dot' | 'hex')[] = ['shield', 'dot', 'hex'];
    return {
      x: Math.random() * window.innerWidth,
      y: Math.random() * window.innerHeight,
      vx: (Math.random() - 0.5) * 0.3,
      vy: -0.2 - Math.random() * 0.3,
      size: 3 + Math.random() * 4,
      opacity: 0.1 + Math.random() * 0.3,
      type: types[Math.floor(Math.random() * types.length)],
      rotation: Math.random() * Math.PI * 2,
      rotationSpeed: (Math.random() - 0.5) * 0.02
    };
  }

  private animate() {
    if (!this.ctx || this.prefersReducedMotion()) return;

    const canvas = this.canvasRef.nativeElement;
    this.ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Get active nodes for particle attraction
    const activeNodes = this.nodes().filter(n => n.active);
    const attractionStrength = this.isMobile ? this.ATTRACTION_STRENGTH_MOBILE : this.ATTRACTION_STRENGTH;

    // Update and draw particles with attraction
    for (let i = 0; i < this.particles.length; i++) {
      const p = this.particles[i];

      // Apply attraction to nearest active node
      if (activeNodes.length > 0) {
        let nearestDist = Infinity;
        let nearestNode = activeNodes[0];

        for (const node of activeNodes) {
          // Convert SVG coordinates to canvas coordinates
          const nodeX = (node.x / 1920) * canvas.width;
          const nodeY = (node.y / 1080) * canvas.height;
          const dx = nodeX - p.x;
          const dy = nodeY - p.y;
          const dist = Math.sqrt(dx * dx + dy * dy);

          if (dist < nearestDist) {
            nearestDist = dist;
            nearestNode = node;
          }
        }

        // Apply attraction force
        const targetX = (nearestNode.x / 1920) * canvas.width;
        const targetY = (nearestNode.y / 1080) * canvas.height;
        p.vx += (targetX - p.x) * attractionStrength;
        p.vy += (targetY - p.y) * attractionStrength;

        // Dampen velocity to prevent runaway acceleration
        p.vx *= 0.99;
        p.vy *= 0.99;

        // If particle gets very close to node, fade it out and respawn
        if (nearestDist < 30) {
          p.opacity -= 0.02;
          if (p.opacity <= 0) {
            // Respawn at edge
            p.x = Math.random() * canvas.width;
            p.y = canvas.height + 50;
            p.vx = (Math.random() - 0.5) * 0.3;
            p.vy = -0.2 - Math.random() * 0.3;
            p.opacity = 0.1 + Math.random() * 0.3;
          }
        }
      }

      // Update position
      p.x += p.vx;
      p.y += p.vy;
      p.rotation += p.rotationSpeed;

      // Reset if off screen
      if (p.y < -50 || p.x < -50 || p.x > canvas.width + 50) {
        p.x = Math.random() * canvas.width;
        p.y = canvas.height + 50;
        p.vx = (Math.random() - 0.5) * 0.3;
        p.vy = -0.2 - Math.random() * 0.3;
        p.opacity = 0.1 + Math.random() * 0.3;
      }

      // Draw particle
      this.drawParticle(p);
    }

    // Update and draw data packets
    this.updateAndDrawPackets(canvas);

    this.animationFrameId = requestAnimationFrame(() => this.animate());
  }

  private drawParticle(p: Particle) {
    if (!this.ctx) return;

    this.ctx.save();
    this.ctx.translate(p.x, p.y);
    this.ctx.rotate(p.rotation);
    this.ctx.globalAlpha = p.opacity;

    if (p.type === 'dot') {
      // Simple glowing dot
      this.ctx.beginPath();
      this.ctx.arc(0, 0, p.size, 0, Math.PI * 2);
      this.ctx.fillStyle = '#FF7A30';
      this.ctx.fill();
    } else if (p.type === 'shield') {
      // Shield outline
      this.ctx.strokeStyle = '#FF7A30';
      this.ctx.lineWidth = 1;
      this.ctx.beginPath();
      const s = p.size * 2;
      this.ctx.moveTo(0, -s);
      this.ctx.lineTo(-s * 0.8, -s * 0.5);
      this.ctx.lineTo(-s * 0.8, s * 0.3);
      this.ctx.quadraticCurveTo(0, s * 1.2, 0, s * 1.2);
      this.ctx.quadraticCurveTo(0, s * 1.2, s * 0.8, s * 0.3);
      this.ctx.lineTo(s * 0.8, -s * 0.5);
      this.ctx.closePath();
      this.ctx.stroke();
    } else if (p.type === 'hex') {
      // Hexagon
      this.ctx.strokeStyle = '#3B82F6';
      this.ctx.lineWidth = 1;
      this.ctx.beginPath();
      for (let i = 0; i < 6; i++) {
        const angle = (Math.PI / 3) * i - Math.PI / 6;
        const x = Math.cos(angle) * p.size;
        const y = Math.sin(angle) * p.size;
        if (i === 0) {
          this.ctx.moveTo(x, y);
        } else {
          this.ctx.lineTo(x, y);
        }
      }
      this.ctx.closePath();
      this.ctx.stroke();
    }

    this.ctx.restore();
  }

  // === Data Packet Methods ===

  private generatePackets() {
    const packetCount = this.isMobile ? this.PACKET_COUNT_MOBILE : this.PACKET_COUNT;
    const connections = this.connections();

    if (connections.length === 0) return;

    for (let i = 0; i < packetCount; i++) {
      this.packets.push(this.createPacket());
    }
  }

  private createPacket(): DataPacket {
    const connections = this.connections();
    return {
      connectionIdx: Math.floor(Math.random() * connections.length),
      progress: Math.random(), // Start at random position along line
      speed: 0.003 + Math.random() * 0.004, // Vary speed slightly
      reverse: Math.random() > 0.5
    };
  }

  private updateAndDrawPackets(canvas: HTMLCanvasElement) {
    if (!this.ctx) return;

    const connections = this.connections();
    if (connections.length === 0) return;

    // Ensure we have enough packets
    const targetCount = this.isMobile ? this.PACKET_COUNT_MOBILE : this.PACKET_COUNT;
    while (this.packets.length < targetCount) {
      this.packets.push(this.createPacket());
    }

    for (let i = 0; i < this.packets.length; i++) {
      const packet = this.packets[i];
      const conn = connections[packet.connectionIdx];

      if (!conn) {
        // Connection no longer exists, reassign
        packet.connectionIdx = Math.floor(Math.random() * connections.length);
        continue;
      }

      // Update progress
      packet.progress += packet.reverse ? -packet.speed : packet.speed;

      // If packet reached end, respawn on new connection
      if (packet.progress >= 1 || packet.progress <= 0) {
        packet.connectionIdx = Math.floor(Math.random() * connections.length);
        packet.progress = packet.reverse ? 1 : 0;
        packet.reverse = Math.random() > 0.5;
        packet.speed = 0.003 + Math.random() * 0.004;
      }

      // Calculate position along the line (convert SVG coords to canvas coords)
      const x1 = (conn.x1 / 1920) * canvas.width;
      const y1 = (conn.y1 / 1080) * canvas.height;
      const x2 = (conn.x2 / 1920) * canvas.width;
      const y2 = (conn.y2 / 1080) * canvas.height;

      const x = x1 + (x2 - x1) * packet.progress;
      const y = y1 + (y2 - y1) * packet.progress;

      // Draw packet as glowing dot
      this.drawPacket(x, y);
    }
  }

  private drawPacket(x: number, y: number) {
    if (!this.ctx) return;

    // Outer glow
    this.ctx.save();
    this.ctx.beginPath();
    this.ctx.arc(x, y, 8, 0, Math.PI * 2);
    this.ctx.fillStyle = 'rgba(255, 122, 48, 0.2)';
    this.ctx.fill();

    // Inner bright core
    this.ctx.beginPath();
    this.ctx.arc(x, y, 3, 0, Math.PI * 2);
    this.ctx.fillStyle = '#FF7A30';
    this.ctx.fill();

    this.ctx.restore();
  }
}
