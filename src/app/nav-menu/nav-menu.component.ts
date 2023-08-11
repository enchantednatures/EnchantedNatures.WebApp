import { Component, OnInit } from '@angular/core';
import { animate, state, style, transition, trigger } from "@angular/animations";
import { Category } from '../category';
import { PhotoService } from '../photo.service';

@Component({
  selector: 'app-nav-menu',
  templateUrl: './nav-menu.component.html',
  styleUrls: ['./nav-menu.component.css'],
  animations: [
    // animation triggers go here
    trigger('openClose', [
      // ...
      state('open', style({
        width: '*',
      })),
      state('closed', style({
        width: '0px',
      })),
      transition('open => closed', [
        animate('1s')
      ]),
      transition('closed => open', [
        animate('0.5s')
      ]),
    ]),
  ]
})
export class NavMenuComponent implements OnInit {
  get isExpanded(): boolean {
    return true;
  }

  public categories: Category[];

  constructor(
    private photoService: PhotoService,
  ) {
    this.categories = [];
  }

  ngOnInit() {
    this.photoService.getCategories().subscribe(c => this.categories = c);
    // if (this.categories.length != 0) { return; }
  }
}

