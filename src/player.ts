import Phaser from "phaser";

function shortenNpub(pubkey: string) {
  // Show npub1 prefix + first 5 + last 5 chars
  return `npub1${pubkey.slice(5, 10)}...${pubkey.slice(-5)}`;
}

export class Player {
  sprite: Phaser.GameObjects.Rectangle;
  label: Phaser.GameObjects.Text;
  pubkey: string;
  isSelf: boolean;

  constructor(scene: Phaser.Scene, x: number, y: number, pubkey: string, isSelf = false) {
    this.pubkey = pubkey;
    this.isSelf = isSelf;

    const color = isSelf ? 0xff0000 : 0x0000ff;
    this.sprite = scene.add.rectangle(x, y, isSelf ? 30 : 20, isSelf ? 30 : 20, color);

    this.label = scene.add
      .text(
        x,
        y - 30,
        isSelf ? shortenNpub(pubkey) : shortenNpub(pubkey),
        {
          fontSize: "12px",
          color: "#ffffff",
          backgroundColor: "#00000088",
        }
      )
      .setOrigin(0.5);
  }

  setPosition(x: number, y: number) {
    this.sprite.setPosition(x, y);
    this.label.setPosition(x, y - 30);
  }

  get x() {
    return this.sprite.x;
  }

  get y() {
    return this.sprite.y;
  }
}
