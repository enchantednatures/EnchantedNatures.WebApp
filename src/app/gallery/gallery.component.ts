import { Component, Inject, OnInit } from '@angular/core';
import { HttpClient, HttpParams } from "@angular/common/http";
import { ActivatedRoute, ParamMap, Router } from '@angular/router';
import { switchMap, tap } from "rxjs/operators";
import { NavService } from '../nav.service';
import { PhotoService } from '../photo.service';
import { Photo } from '../photo';


@Component({
  selector: 'app-gallery',
  templateUrl: './gallery.component.html',
  styleUrls: ['./gallery.component.css']
})
export class GalleryComponent implements OnInit {
  private _entityId: number = 1;
  public photos: Photo[];

  constructor(
    private photoService: PhotoService,
    private route: ActivatedRoute,
    private nav: NavService
  ) {
    this.photos = [];
  }


  ngOnInit() {
    // Or as an alternative, with slightly different execution...
    this.route.paramMap.subscribe((params: ParamMap) => {
      this._entityId = +(params.get('id') ?? -1);
      this.getPhotosForCategory()

    });
    this.getPhotosForCategory()
  }

  getPhotosForCategory() {
    this.photoService.getCategoryPhotos(1).subscribe(c => this.photos = c.photos)

  }
  onScroll(event: WheelEvent) {
    this.nav.onScroll(event)
  }
}
