import {Component, OnInit} from '@angular/core';
import {NavService} from "../nav.service";
import {ActivatedRoute, NavigationEnd, Router} from "@angular/router";
import {Photo} from "../photo";
import {PhotoService} from "../photo.service";
import {filter} from "rxjs/operators";

@Component({
  selector: 'app-studio',
  templateUrl: './studio.component.html',
  styleUrls: ['./studio.component.css']
})
export class StudioComponent implements OnInit {
  public photo: Photo | undefined;
  loading = true;
  error: any;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private nav: NavService,
    private photoService: PhotoService,
  ) {
  }

  ngOnInit(): void {
    this.router.events
      .subscribe((_) => {
        // code goes here...
        let categoryId: number = 1;
        this.route.queryParams.subscribe(params => {
          categoryId = params['photo'];
        });
        this.getPhoto(categoryId);
      });
  }


  getPhoto(id: number) {
    this.photoService.getPhoto(id).subscribe(c => this.photo = c);
  }
}
