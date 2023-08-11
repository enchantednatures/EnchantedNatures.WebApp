import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http'
import { Category } from './category';
import { Observable, catchError, map, of, tap } from 'rxjs';
import { CategoryPhotos } from './category-photos';

@Injectable({
  providedIn: 'root'
})
export class PhotoService {

  constructor(
    private http: HttpClient
  ) { }

  getCategories(): Observable<Category[]> {
    return this.http
      .get<Category[]>(
        "api/categories",
        { responseType: "json" },
      ).pipe(
        map(h => h),
        tap(
          h => {
            const outcome = h ? 'fetched' : 'did not find';
          }),
        catchError(this.handleError<Category[]>("unable to get catories"))
      );
  }

  getCategoryPhotos(id: number): Observable<CategoryPhotos> {
    return this.http
      .get<CategoryPhotos>(
        `api/categories/${ id }`,
        { responseType: "json" },
      ).pipe(
        map(h => h),
        tap(
          h => {
            const outcome = h ? 'fetched' : 'did not find';
          }),
        catchError(this.handleError<CategoryPhotos>("unable to get catories"))
      );
  }

  private handleError<T>(operation = 'operation', result?: T) {
    return (error: any): Observable<T> => {
      console.error(error); // log to console instead
      this.log(`${operation} failed: ${error.message}`);
      return of(result as T);
    };
  }

  private log(message: string) { }
}
