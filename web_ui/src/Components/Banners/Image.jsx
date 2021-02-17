import React, { useState, useEffect, useCallback } from "react";

function BannerImage(props) {
  const [currentSrc, setCurrentSrc] = useState();
  const [show, setShow] = useState(false);

  const [loaded, setLoaded] = useState(false);
  const [error, setErr] = useState(false);

  useEffect(() => {
    if (props.src !== currentSrc) {
      setShow(false);
      setLoaded(false);
    }
  }, [props.src])

  const swapSrc = useCallback(e => {
    if (e.animationName !== props.hideAnimationName) return;

    setErr(false);

    if (props.src !== currentSrc) {
      const img = new Image();

      img.onload = (e) => {
        setLoaded(true);
        setShow(true);
        setCurrentSrc(img.src);
      };

      img.onerror = () => {
        setLoaded(true);
        setShow(true);
        setErr(true);
      };

      img.src = new RegExp("/^(?:\/|[a-z]+:\/\/)/").test(props.src)
        ? props.src : `//${window.host}:${window.backend_port}/${props.src}`;
    }
  }, [props.src]);

  return (
    <div
      className={`image-wrapper show-${show && loaded}`}
      onAnimationEnd={swapSrc}
    >
      {!error && (
        <img
          src={currentSrc}
          key={currentSrc}
          aria-label="banner"
        />
      )}
      {error && <div className="placeholder"/>}
    </div>
  )
}

export default BannerImage;
