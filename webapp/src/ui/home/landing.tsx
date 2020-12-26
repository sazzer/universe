import React, { Suspense } from "react";
import { Spinner } from "../utilities/spinner";
import landing from "./landing.jpg";

export const Landing: React.FC = ({ children }) => {
  return (
    <div className="row">
      <div className="col-12 col-lg-3 order-lg-3">
        <Suspense fallback={<Spinner />}>{children}</Suspense>
      </div>
      <div className="col-12 col-lg-9">
        <div className="card border-0">
          <img src={landing} width="100%" alt="" className="card-img-top" />
          <div className="card-body">
            <h5 className="card-title">The Path Less Travelled</h5>
            <p className="card-text">By Nino Is</p>
          </div>
        </div>
      </div>
    </div>
  );
};
