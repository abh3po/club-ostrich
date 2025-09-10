import Phaser from "phaser";
import { Player } from "./player";

export class GameScene extends Phaser.Scene {
  player!: Player;
  cursors!: Phaser.Types.Input.Keyboard.CursorKeys;
  others = new Map<string, Player>();
  sendPosition!: (x: number, y: number) => void;

  preload() {
    this.load.image("room1", "assets/room1.png");
  }

  create() {
    this.add.image(400, 300, "room1").setDisplaySize(800, 600);

    // Local player, using pubkey from main.ts
    const selfPubkey = (this as any).selfPubkey || "you";
    this.player = new Player(this, 200, 200, selfPubkey, true);

    // Input
    this.cursors = this.input.keyboard!.createCursorKeys();
  }

  update() {
    let moved = false;
    const speed = 200 * (this.game.loop.delta / 1000);

    let newX = this.player.x;
    let newY = this.player.y;

    if (this.cursors.left?.isDown) {
      newX -= speed;
      moved = true;
    }
    if (this.cursors.right?.isDown) {
      newX += speed;
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

    // Clamp to world bounds (800x600)
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
}
