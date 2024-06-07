import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { catchError, map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class BlockService {
  private apiUrl = 'http://localhost:8080/blocks';

  constructor(private http: HttpClient) { }

  getHeights(): Observable<number[]> {
    return this.http.get<any[]>(this.apiUrl).pipe(
      map(data => data.map(item => item.height))
    );
  }

  // getBlocks(): Observable<any[]> {
  //   return this.http.get<any[]>(this.apiUrl).pipe(
  //     map((data: any[]) => {
  //       return data;
  //     }),
  //     catchError(error => {
  //       console.error('Error fetching blocks:', error);
  //       throw error;
  //     })
  //   );
  // }
}
