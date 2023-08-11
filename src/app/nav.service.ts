import {Injectable, OnInit} from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class NavService implements OnInit {
  public isVisible: boolean;

  constructor() {
    this.isVisible = true;
  }

  ngOnInit() {
    this.isVisible = true;
  }

  hide() {
    this.isVisible = false;
  }

  show() {
    this.isVisible = true;
  }

  toggle() {
    // this.visible = !this.visible;
    if (this.isVisible) {
      this.hide();
    } else {
      this.show();
    }
  }

  onScroll(event: WheelEvent) {
    if (event.deltaY > 0) {
      this.hide();
    } else {
      this.show();

    }
  }
}

