import Phaser from "phaser";
import { Player } from "./player";

export class GameScene extends Phaser.Scene {
  player!: Player;
  cursors!: Phaser.Types.Input.Keyboard.CursorKeys;
  others = new Map<string, Player>();
  sendPosition!: (x: number, y: number) => void;
  targetX: number | null = null;
  targetY: number | null = null;

  preload() {
    this.load.image("room1", "assets/room1.png");
    this.load.image("ostrich_player", "assets/ostrich_player.png");
    this.load.image("ostrich_other", "assets/ostrich_other.png");
  }

  create() {
    this.add.image(400, 300, "room1").setDisplaySize(800, 600);

    // Local player, using pubkey from main.ts
    const selfPubkey = (this as any).selfPubkey || "you";
    this.player = new Player(this, 200, 200, selfPubkey, true);

    // Input
    this.cursors = this.input.keyboard!.createCursorKeys();
    this.input.on("pointerdown", (pointer: Phaser.Input.Pointer) => {
      this.targetX = pointer.x;
      this.targetY = pointer.y;
    });    
  }

  update() {
    let moved = false;
    const speed = 200 * (this.game.loop.delta / 1000);
  
    let newX = this.player.x;
    let newY = this.player.y;
  
    if (this.sys.game.device.os.desktop) {
      // --- Desktop: Arrow keys ---
      if (this.cursors.left?.isDown) {
        newX -= speed;
        this.player.sprite.setFlipX(true);
        moved = true;
      }
      if (this.cursors.right?.isDown) {
        newX += speed;
        this.player.sprite.setFlipX(false);
        moved = true;
      }
      if (this.cursors.up?.isDown) {
        newY -= speed;
        moved = true;
      }
      if (this.cursors.down?.isDown) {
        newY += speed;
        moved = true;
      }
    } else {
      // --- Mobile: Move toward tap target ---
      if (this.targetX !== null && this.targetY !== null) {
        const dx = this.targetX - this.player.x;
        const dy = this.targetY - this.player.y;
        const dist = Math.sqrt(dx * dx + dy * dy);
  
        if (dist > 4) { // close enough, stop
          const angle = Math.atan2(dy, dx);
          newX += Math.cos(angle) * speed;
          newY += Math.sin(angle) * speed;
  
          // Flip ostrich to face direction
          this.player.sprite.setFlipX(dx < 0);
  
          moved = true;
        } else {
          this.targetX = null;
          this.targetY = null;
        }
      }
    }
  
    // Clamp to bounds
    newX = Phaser.Math.Clamp(newX, 0, 800);
    newY = Phaser.Math.Clamp(newY, 0, 600);
  
    this.player.setPosition(newX, newY);
  
    if (moved && this.sendPosition) {
      this.sendPosition(newX, newY);
    }
  }
  
  

  updateOther(pubkey: string, x: number, y: number) {
    if (!this.others.has(pubkey)) {
      this.others.set(pubkey, new Player(this, x, y, pubkey, false));
    } else {
      this.others.get(pubkey)!.setPosition(x, y);
    }
  }

  showMessage(pubkey: string, msg: string) {
    const player = pubkey === this.player.pubkey ? this.player : this.others.get(pubkey);
    if (!player) return;
  
    const bubble = this.add.text(player.x, player.y - 80, msg, {
      fontSize: "14px",
      color: "#000000",
      backgroundColor: "#ffffffaa",
      padding: { x: 5, y: 3 },
    }).setOrigin(0.5);
  
    // Destroy after 3s
    this.time.delayedCall(3000, () => bubble.destroy());
  }
}

