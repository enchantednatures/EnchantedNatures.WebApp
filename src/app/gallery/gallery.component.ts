import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, NavigationEnd, Router} from '@angular/router';
import {filter} from "rxjs/operators";
import {NavService} from '../nav.service';
import {PhotoService} from '../photo.service';
import {Photo} from '../photo';


@Component({
  selector: 'app-gallery',
  templateUrl: './gallery.component.html',
  styleUrls: ['./gallery.component.css']
})
export class GalleryComponent implements OnInit {
  public photos: Photo[];

  constructor(
    private photoService: PhotoService,
    private route: ActivatedRoute,
    private router: Router,
    private nav: NavService
  ) {
    this.photos = [];
  }

  ngOnChanges() {
    let categoryId: number = 1;
    this.route.queryParams.subscribe(params => {
      categoryId = params['category'];
    });
    this.getPhotosForCategory(categoryId);
  }

  ngOnInit() {
    let categoryId: number = 1;
    this.route.queryParams.subscribe(params => {
      categoryId = params['category'];
    });
    this.getPhotosForCategory(categoryId);
    this.router.events
      .subscribe((_) => {
        // code goes here...

        this.route.queryParams.subscribe(params => {
          categoryId = params['category'];
        });
        this.getPhotosForCategory(categoryId);
      });
  }

  getPhotosForCategory(id: number) {
    this.photoService.getCategoryPhotos(id).subscribe(c => this.photos = c.photos)
  }

  onScroll(event: WheelEvent) {
    this.nav.onScroll(event)
  }
}
